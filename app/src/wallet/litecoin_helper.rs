use crate::entity::litecoin_wallet::{self, ActiveModel as LitecoinWalletActiveModel};
use crate::entity::tokens::{self, ActiveModel as TokensActiveModel};
use crate::wallet::litecoin::{LitecoinError, LitecoinWallet};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Custom error type for Litecoin helper functions.
#[derive(Debug)]
pub enum LitecoinHelperError {
    Litecoin(LitecoinError),
    Db(sea_orm::DbErr),
    NotFound(String),
}

impl Display for LitecoinHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LitecoinHelperError::Litecoin(err) => write!(f, "Litecoin error: {}", err),
            LitecoinHelperError::Db(err) => write!(f, "Database error: {}", err),
            LitecoinHelperError::NotFound(msg) => write!(f, "Not Found: {}", msg),
        }
    }
}

impl From<LitecoinError> for LitecoinHelperError {
    fn from(err: LitecoinError) -> Self {
        LitecoinHelperError::Litecoin(err)
    }
}

impl From<sea_orm::DbErr> for LitecoinHelperError {
    fn from(err: sea_orm::DbErr) -> Self {
        LitecoinHelperError::Db(err)
    }
}

/// Ensures an account index exists for the user.
/// If not, finds the highest used account index in the database and uses the next one.
/// Also creates a change address (is_change = true) at address_index 0.
/// Returns the account index.
pub async fn ensure_litecoin_account_index_for_user(
    user_row: &tokens::Model,
    litecoin_wallet_client: &LitecoinWallet,
    conn: &DatabaseConnection,
) -> Result<u32, LitecoinHelperError> {
    if let Some(account_index) = user_row.litecoin_account_index {
        Ok(account_index as u32)
    } else {
        // Get the highest used account index from the database
        let max_account_index = litecoin_wallet::Entity::find()
            .select_only()
            .column_as(litecoin_wallet::Column::AccountIndex.max(), "max_index")
            .into_model::<MaxIndexResult>()
            .one(conn)
            .await?
            .and_then(|r| r.max_index)
            .unwrap_or(0);

        let new_account_index = (max_account_index + 1) as u32;

        // Create change address at index 0 (is_change = true)
        let change_address = litecoin_wallet_client
            .derive_address(new_account_index, 0)
            .await?;

        let blockchain_height = litecoin_wallet_client.get_block_height().await?.height as i32;

        let new_litecoin_wallet_entry = LitecoinWalletActiveModel {
            account_index: Set(new_account_index as i32),
            address_index: Set(0),
            wallet_address: Set(change_address.address.clone()),
            is_available: Set(Some(false)), // change addresses are not available for deposits
            is_change: Set(true),
            blockchain_height: Set(blockchain_height),
            ..Default::default()
        };
        new_litecoin_wallet_entry.insert(conn).await?;

        // Update the user's token entry with the new account index
        let mut token_active_model: TokensActiveModel = user_row.clone().into();
        token_active_model.litecoin_account_index = Set(Some(new_account_index as i32));
        token_active_model.update(conn).await?;

        Ok(new_account_index)
    }
}

/// Retrieves a free Litecoin deposit address for a given account index.
/// It first checks for an available address in the database. If none exist,
/// it creates a new one via the Litecoin API and stores it.
pub async fn get_free_litecoin_address_with_account_index(
    account_index: u32,
    litecoin_wallet_client: &LitecoinWallet,
    conn: &DatabaseConnection,
) -> Result<String, LitecoinHelperError> {
    // Get blockchain height
    let blockchain_height = litecoin_wallet_client.get_block_height().await?.height as i32;

    // 1. Search for an existing available address in the database
    if let Some(available_address_model) = litecoin_wallet::Entity::find()
        .filter(litecoin_wallet::Column::AccountIndex.eq(account_index as i32))
        .filter(litecoin_wallet::Column::IsAvailable.eq(true))
        .filter(litecoin_wallet::Column::IsChange.eq(false))
        .one(conn)
        .await?
    {
        // 2. If found, check current balance and update initial_balance
        let address = &available_address_model.wallet_address;
        let balance_response = litecoin_wallet_client
            .get_balance(&[address.clone()])
            .await?;

        let confirmed_balance = balance_response
            .get(address)
            .map(|e| e.confirmed.to_string())
            .unwrap_or("0".to_string());

        let mut active_model: LitecoinWalletActiveModel = available_address_model.clone().into();
        active_model.is_available = Set(Some(false));
        active_model.blockchain_height = Set(blockchain_height);
        active_model.initial_balance = Set(Some(confirmed_balance));
        // NOTE: unconfirmed balance is not checked here — the finalizer is responsible
        // for ensuring an address has no unconfirmed balance before setting is_available = true
        active_model.update(conn).await?;

        Ok(address.clone())
    } else {
        // 3. No available address — derive the next one
        let max_address_index = litecoin_wallet::Entity::find()
            .filter(litecoin_wallet::Column::AccountIndex.eq(account_index as i32))
            .filter(litecoin_wallet::Column::IsChange.eq(false))
            .select_only()
            .column_as(litecoin_wallet::Column::AddressIndex.max(), "max_index")
            .into_model::<MaxIndexResult>()
            .one(conn)
            .await?
            .and_then(|r| r.max_index)
            .unwrap_or(0);

        let new_address_index = (max_address_index + 1) as u32;

        // Derive the new address
        let derive_response = litecoin_wallet_client
            .derive_address(account_index, new_address_index)
            .await?;

        let new_address = derive_response.address;

        // Check balance of the newly derived address
        let balance_response = litecoin_wallet_client
            .get_balance(&[new_address.clone()])
            .await?;

        let confirmed_balance = balance_response
            .get(&new_address)
            .map(|e| e.confirmed.to_string())
            .unwrap_or("0".to_string());

        // Insert into the database with is_available = false
        let new_litecoin_wallet_entry = LitecoinWalletActiveModel {
            account_index: Set(account_index as i32),
            address_index: Set(new_address_index as i32),
            wallet_address: Set(new_address.clone()),
            is_available: Set(Some(false)),
            is_change: Set(false),
            blockchain_height: Set(blockchain_height),
            initial_balance: Set(Some(confirmed_balance)),
            ..Default::default()
        };
        new_litecoin_wallet_entry.insert(conn).await?;

        Ok(new_address)
    }
}

#[derive(sea_orm::FromQueryResult)]
struct MaxIndexResult {
    pub max_index: Option<i32>,
}
