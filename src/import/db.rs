use anyhow::{Context, Result};
use sqlx::{QueryBuilder, Row, SqlitePool};
use std::collections::HashMap;

use super::triple::LdTabTriple;

const SQLITE_MAX_VARIABLE_NUMBER: usize = 999;
const NUM_COLUMNS: usize = 8;

pub(crate) async fn load_prefix_map(pool: &SqlitePool) -> Result<HashMap<String, String>> {
    let rows = sqlx::query(
        r#"
        SELECT * FROM prefix
        "#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to fetch rows from the table")?;

    let mut map = HashMap::new();
    for row in rows {
        let base: String = row.get("base");
        let prefix: String = row.get("prefix");
        map.insert(base, prefix);
    }
    Ok(map)
}

pub(crate) async fn insert_triples_to_db(triples: &[LdTabTriple], pool: &SqlitePool) -> Result<()> {
    let mut start = 0;
    while start < triples.len() {
        let end = std::cmp::min(
            start + SQLITE_MAX_VARIABLE_NUMBER / NUM_COLUMNS,
            triples.len(),
        );
        let chunk = &triples[start..end];

        let mut query_builder = QueryBuilder::new(
            "INSERT INTO statement (assertion, retraction, graph, subject, predicate, object, datatype, annotation) ",
        );

        query_builder.push_values(chunk, |mut b, triple| {
            b.push_bind(triple.assertion)
                .push_bind(triple.retraction)
                .push_bind(&triple.graph)
                .push_bind(&triple.subject)
                .push_bind(&triple.predicate)
                .push_bind(&triple.object)
                .push_bind(&triple.datatype)
                .push_bind(&triple.annotation);
        });

        query_builder
            .build()
            .execute(pool)
            .await
            .context("Failed to insert ldtab_triples into the database")?;

        start = end;
    }
    Ok(())
}