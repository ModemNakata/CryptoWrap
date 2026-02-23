use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tokens")
                    .if_not_exists()
                    .col(
                        uuid("id")
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()"))
                            .not_null(),
                    )
                    .col(string("token_hash").string_len(64).not_null())
                    .col(
                        date_time("created_at")
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tokens").to_owned())
            .await
    }
}
