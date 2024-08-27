use anyhow::{Context, Result};
use clap::ArgMatches;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs::File;
use std::path::Path;

pub async fn init(sub_matches: &ArgMatches) -> Result<()> {
    let input = sub_matches.get_one::<String>("input");
    let connection = sub_matches.get_one::<String>("connection");

    let database = match (input, connection) {
        (Some(input), None) => input,
        (None, Some(connection)) => connection,
        (None, None) => {
            anyhow::bail!("You must provide either a database name or a database connection.")
        }
        //this case should not happen (clap should prevent it - see 'conflicts_with' in clap definition)
        (Some(_), Some(_)) => {
            anyhow::bail!("Both input and connection are provided, but only one should be.")
        }
    };

    //TODO: this currently only works for SQLite
    create_database(&database).await?;

    Ok(())
}

async fn create_database(database: &str) -> Result<SqlitePool> {
    // Check if the database file already exists
    if !Path::new(database).exists() {
        println!(
            "Database file does not exist. Creating database file at {}...",
            database
        );
        File::create(database)
            .with_context(|| format!("Failed to create database file at {}", database))?;
    } else {
        println!("Database file already exists at {}.", database);
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database)
        .await
        .context("Failed to connect to the database")?;

    // Create tables ldtab, prefix, and statement
    sqlx::query(
        r#"
        BEGIN TRANSACTION;
        CREATE TABLE ldtab(
          'key' TEXT PRIMARY KEY,
          'value' TEXT
        );
        INSERT INTO ldtab VALUES('ldtab version','0.0.1');
        INSERT INTO ldtab VALUES('schema version','0');
        CREATE TABLE prefix (
          'prefix' TEXT PRIMARY KEY,
          'base' TEXT NOT NULL
        );
        CREATE TABLE statement (
          'assertion' INTEGER NOT NULL,
          'retraction' INTEGER NOT NULL DEFAULT 0,
          'graph' TEXT NOT NULL,
          'subject' TEXT NOT NULL,
          'predicate' TEXT NOT NULL,
          'object' TEXT NOT NULL,
          'datatype' TEXT NOT NULL,
          'annotation' TEXT
        );
        COMMIT;
        "#,
    )
    .execute(&pool)
    .await
    .context("Failed to create the table")?;

    Ok(pool)
}
