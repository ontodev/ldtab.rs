use anyhow::{Context, Result};
use clap::ArgMatches;
use sqlx::sqlite::SqlitePoolOptions;
use std::fs::File;

pub async fn prefix(sub_matches: &ArgMatches) -> Result<()> {
    let database = sub_matches.get_one::<String>("database").unwrap();
    let prefixes = sub_matches.get_one::<String>("prefixes").unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database)
        .await
        .context("Failed to connect to the database")?;

    // Open the TSV file
    let file = File::open(prefixes)?;

    // Create a CSV reader with tab delimiter
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(file);

    // Iterate over the records in the TSV file
    for result in rdr.records() {
        let record = result?;

        // Get the base and prefix from the record
        let prefix = &record[0];
        let base = &record[1];

        // Insert into the database
        sqlx::query("INSERT INTO prefix (base, prefix) VALUES ($1, $2)")
            .bind(base)
            .bind(prefix)
            .execute(&pool)
            .await
            .context("Failed to insert ldtab_triples into the database")?;
    }

    Ok(())
}
