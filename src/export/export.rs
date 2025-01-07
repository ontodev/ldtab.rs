use anyhow::{Context, Result};
use clap::ArgMatches;
use sqlx::{sqlite::SqlitePoolOptions, Row};
use std::fs::OpenOptions;
use std::io::Write;

fn escape_whitespace(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\t' => "\\t".to_string(),
            //'\r' => "\\r".to_string(),
            //'\x08' => "\\b".to_string(),
            //'\x0C' => "\\f".to_string(),
            //'\x0B' => "\\v".to_string(),
            //' '  => "\\s".to_string(), // Optional: Escape space as '\\s'
            _ => c.to_string(),
        })
        .collect()
}

pub async fn export(sub_matches: &ArgMatches) -> Result<()> {
    let database = sub_matches.get_one::<String>("database");
    let output = sub_matches.get_one::<String>("output");

    //these unwraps are safe
    let database_path = database.unwrap();
    let output_path = output.unwrap();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_path)
        .await
        .context("Failed to connect to the database")?;

    //load data
    let rows = sqlx::query(
        r#"
        SELECT * FROM statement
        "#,
    )
    .fetch_all(&pool)
    .await
    .context("Failed to fetch rows from the table")?;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)?;

    let header = "assertion\tretraction\tgraph\tsubject\tpredicate\tobject\tdatatype\tannotation";
    file.write_all(header.as_bytes())?;
    file.write_all(b"\n")?;

    for row in rows {
        let assertion: i32 = row.get("assertion");
        let retraction: i32 = row.get("retraction");
        let graph: String = row.get("graph");
        let subject: String = row.get("subject");
        let predicate: String = row.get("predicate");
        let object: String = escape_whitespace(row.get("object"));
        let datatype: String = row.get("datatype");
        let annotation: String = row.get("annotation");

        let line = format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            assertion, retraction, graph, subject, predicate, object, datatype, annotation
        );

        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
