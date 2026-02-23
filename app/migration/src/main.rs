use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}

// cargo install sea-orm-cli
// sea-orm-cli migrate up -u <DATABASE_URL_FROM_DOT_ENV>

// generate entities
//
// # Generate entity files of database `bakery` to `./src/entity`
// sea-orm-cli generate entity \
//     --database-url protocol://username:password@localhost/bakery \
//     --output-dir ./src/entity
//
// https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli/
