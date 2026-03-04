use crate::entity::monero_wallet::{self, ActiveModel as MoneroWalletActiveModel};
use crate::entity::tokens::{self, ActiveModel as TokensActiveModel};
use crate::wallet::monero::{self, MoneroError, MoneroWallet};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::fmt::{Display, Formatter, Result as FmtResult};
use uuid::Uuid;

/// Custom error type for Monero helper functions.
#[derive(Debug)]
pub enum MoneroHelperError {
    MoneroRpc(MoneroError),
    Db(sea_orm::DbErr),
    NotFound(String),
}

impl Display for MoneroHelperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MoneroHelperError::MoneroRpc(err) => write!(f, "Monero RPC error: {}", err),
            MoneroHelperError::Db(err) => write!(f, "Database error: {}", err),
            MoneroHelperError::NotFound(msg) => write!(f, "Not Found: {}", msg),
        }
    }
}

impl From<MoneroError> for MoneroHelperError {
    fn from(err: MoneroError) -> Self {
        MoneroHelperError::MoneroRpc(err)
    }
}

impl From<sea_orm::DbErr> for MoneroHelperError {
    fn from(err: sea_orm::DbErr) -> Self {
        MoneroHelperError::Db(err)
    }
}

/// Ensures a major wallet index exists for the user.
/// If not, a new account is created via Monero RPC and stored in the database.
/// Returns the major wallet index.
pub async fn ensure_monero_major_wallet_index_for_user(
    user_row: &tokens::Model,
    monero_wallet_client: &MoneroWallet,
    conn: &DatabaseConnection,
) -> Result<u32, MoneroHelperError> {
    if let Some(major_index) = user_row.monero_major_index {
        Ok(major_index as u32)
    } else {
        // Create a new account in Monero wallet
        let create_account_response = monero::create_account(
            monero_wallet_client,
            // Some(&format!("user_{}", user_row.id)), // Label for the account
        )
        .await?; // Convert MoneroError to MoneroHelperError

        let new_major_index = create_account_response.account_index;
        // we can also save major index wallet address (account) which is probably the `change` address at index 0, first subaddress is index 1
        // so 0 is primary address as well/too

        // Update the user's token entry in the database with the new major index
        let mut token_active_model: TokensActiveModel = user_row.clone().into();
        token_active_model.monero_major_index = Set(Some(new_major_index as i32));
        token_active_model.update(conn).await?; // Convert DbErr to MoneroHelperError

        Ok(new_major_index)
    }
}

/// Retrieves a free Monero subaddress for a given major account index.
/// It first checks for an available address in the database. If none exist,
/// it creates a new one via Monero RPC and stores it.
pub async fn get_free_monero_subaddress_with_major_index(
    major_index: u32,
    monero_wallet_client: &MoneroWallet,
    conn: &DatabaseConnection,
) -> Result<String, MoneroHelperError> {
    // 1. Search for an existing available address in the database
    if let Some(mut available_address_model) = monero_wallet::Entity::find()
        .filter(monero_wallet::Column::MajorIndex.eq(major_index as i32))
        .filter(monero_wallet::Column::IsAvailable.eq(true))
        .one(conn)
        .await?
    {
        // 2. If found, update its status to not available and return the address
        let mut active_model: MoneroWalletActiveModel = available_address_model.clone().into();
        active_model.is_available = Set(false);
        active_model.update(conn).await?; // No need to set last_used_at manually if default is handled by DB
        Ok(available_address_model.wallet_address)
    } else {
        let blockchain_height = monero::get_height(monero_wallet_client).await?;

        // 3. If no available address, create a new one via Monero RPC
        let create_address_response = monero::create_address(
            monero_wallet_client,
            major_index,
            // None, // No specific label for the subaddress
            // Some(1), // Create one address
        )
        .await?;

        let new_address = create_address_response.address;

        let new_minor_index = create_address_response.address_index;

        // 4. Insert the new address into the database with is_available = false
        let new_monero_wallet_entry = MoneroWalletActiveModel {
            major_index: Set(major_index as i32),
            minor_index: Set(new_minor_index as i32),
            wallet_address: Set(new_address.clone()),
            is_available: Set(false),
            blockchain_height: Set(blockchain_height.height),
            ..Default::default() // Let the database handle created_at and last_used_at defaults
        };
        new_monero_wallet_entry.insert(conn).await?;

        Ok(new_address)
    }
}
