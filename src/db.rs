//! Database operations module

use rust_decimal::Decimal;
use std::collections::HashMap;
use tokio_postgres::Client;

use crate::constants::db::MAX_ROWS_PER_TABLE;
use crate::types::{RowData, TableStats};

/// Get primary key column name
pub async fn get_primary_key(client: &Client, schema: &str, table: &str) -> Option<String> {
    let query = r#"
        SELECT a.attname
        FROM pg_index i
        JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY(i.indkey)
        WHERE i.indrelid = ($1 || '.' || $2)::regclass AND i.indisprimary
        LIMIT 1
    "#;

    client
        .query_opt(query, &[&schema, &table])
        .await
        .ok()
        .flatten()
        .map(|row| row.get::<_, String>(0))
}

/// Fetch all rows from table (up to MAX_ROWS_PER_TABLE rows)
pub async fn fetch_all_rows(
    client: &Client,
    schema: &str,
    table: &str,
) -> Result<Vec<RowData>, Box<dyn std::error::Error + Send + Sync>> {
    let query = format!(
        "SELECT * FROM \"{}\".\"{}\" LIMIT {}",
        schema, table, MAX_ROWS_PER_TABLE
    );
    let rows = client.query(&query, &[]).await?;

    Ok(rows
        .iter()
        .map(|row| {
            row.columns()
                .iter()
                .enumerate()
                .map(|(i, col)| (col.name().to_string(), get_column_value(row, i)))
                .collect()
        })
        .collect())
}

/// Convert PostgreSQL value to string
fn get_column_value(row: &tokio_postgres::Row, idx: usize) -> String {
    // Try each type in order
    try_get_uuid(row, idx)
        .or_else(|| try_get_json(row, idx))
        .or_else(|| try_get_decimal(row, idx))
        .or_else(|| try_get_naive_datetime(row, idx))
        .or_else(|| try_get_datetime_utc(row, idx))
        .or_else(|| try_get_string(row, idx))
        .or_else(|| try_get_i64(row, idx))
        .or_else(|| try_get_i32(row, idx))
        .or_else(|| try_get_f64(row, idx))
        .or_else(|| try_get_bool(row, idx))
        .unwrap_or_else(|| "?".to_string())
}

fn try_get_uuid(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<uuid::Uuid>>(idx).ok().map(|v| {
        v.map(|u| u.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    })
}

fn try_get_json(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<serde_json::Value>>(idx)
        .ok()
        .map(|v| {
            v.map(|j| j.to_string())
                .unwrap_or_else(|| "NULL".to_string())
        })
}

fn try_get_decimal(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<Decimal>>(idx).ok().map(|v| {
        v.map(|d| d.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    })
}

fn try_get_naive_datetime(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<chrono::NaiveDateTime>>(idx)
        .ok()
        .map(|v| {
            v.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "NULL".to_string())
        })
}

fn try_get_datetime_utc(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<chrono::DateTime<chrono::Utc>>>(idx)
        .ok()
        .map(|v| {
            v.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "NULL".to_string())
        })
}

fn try_get_string(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<String>>(idx)
        .ok()
        .map(|v| v.unwrap_or_else(|| "NULL".to_string()))
}

fn try_get_i64(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<i64>>(idx).ok().map(|v| {
        v.map(|n| n.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    })
}

fn try_get_i32(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<i32>>(idx).ok().map(|v| {
        v.map(|n| n.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    })
}

fn try_get_f64(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<f64>>(idx).ok().map(|v| {
        v.map(|n| format!("{:.2}", n))
            .unwrap_or_else(|| "NULL".to_string())
    })
}

fn try_get_bool(row: &tokio_postgres::Row, idx: usize) -> Option<String> {
    row.try_get::<_, Option<bool>>(idx).ok().map(|v| {
        v.map(|b| b.to_string())
            .unwrap_or_else(|| "NULL".to_string())
    })
}

/// Get all available tables
pub async fn get_all_tables(
    client: &Client,
    schema_filter: &str,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let rows = if schema_filter.to_lowercase() == "all" {
        client
            .query(
                "SELECT schemaname, relname FROM pg_stat_user_tables ORDER BY schemaname, relname",
                &[],
            )
            .await?
    } else {
        client
            .query(
                "SELECT schemaname, relname FROM pg_stat_user_tables WHERE schemaname = $1 ORDER BY schemaname, relname",
                &[&schema_filter],
            )
            .await?
    };

    Ok(rows.iter().map(|row| (row.get(0), row.get(1))).collect())
}

/// Get table statistics
pub async fn get_table_stats(
    client: &Client,
    tables: &[(String, String)],
) -> Result<HashMap<(String, String), TableStats>, Box<dyn std::error::Error>> {
    let mut stats = HashMap::new();

    for (schema, table) in tables {
        let row = client
            .query_one(
                "SELECT COALESCE(n_tup_ins, 0), COALESCE(n_tup_upd, 0), COALESCE(n_tup_del, 0) \
                 FROM pg_stat_user_tables WHERE schemaname = $1 AND relname = $2",
                &[schema, table],
            )
            .await?;

        stats.insert(
            (schema.clone(), table.clone()),
            TableStats {
                n_tup_ins: row.get(0),
                n_tup_upd: row.get(1),
                n_tup_del: row.get(2),
            },
        );
    }

    Ok(stats)
}

/// Detect changes in statistics
pub fn has_stats_changes(
    current: &HashMap<(String, String), TableStats>,
    previous: &HashMap<(String, String), TableStats>,
) -> bool {
    current.iter().any(|((schema, table), stats)| {
        let key = (schema.clone(), table.clone());
        if let Some(prev) = previous.get(&key) {
            stats.n_tup_ins > prev.n_tup_ins
                || stats.n_tup_upd > prev.n_tup_upd
                || stats.n_tup_del > prev.n_tup_del
        } else {
            false
        }
    })
}
