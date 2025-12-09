//! Diff calculation module

use std::collections::HashMap;

use crate::types::{RowData, RowDiff};

/// Get primary key value from row (with fallback)
pub fn get_pk_value(row: &RowData, pk_col: &str) -> String {
    // Try the specified PK column
    if let Some(v) = row.get(pk_col) {
        if !v.is_empty() && v != "NULL" {
            return v.clone();
        }
    }

    // Fallback: try common PK column names
    for fallback in &["id", "uuid", "pk"] {
        if let Some(v) = row.get(*fallback) {
            if !v.is_empty() && v != "NULL" {
                return v.clone();
            }
        }
    }

    // Last fallback: use first non-NULL value as identifier
    for (col, val) in row {
        if !val.is_empty() && val != "NULL" {
            return format!("{}:{}", col, val);
        }
    }

    // Absolute fallback: hash of values
    let hash: String = row.values().take(3).cloned().collect::<Vec<_>>().join("_");
    format!("row_{}", hash.chars().take(20).collect::<String>())
}

/// Calculate all diffs between old and new snapshots
pub fn calculate_all_diffs(
    old_rows: &[RowData],
    new_rows: &[RowData],
    pk_col: &str,
) -> Vec<RowDiff> {
    let mut diffs = Vec::new();

    // Build lookup maps by PK
    let old_by_pk: HashMap<String, &RowData> = old_rows
        .iter()
        .map(|r| (get_pk_value(r, pk_col), r))
        .collect();
    let new_by_pk: HashMap<String, &RowData> = new_rows
        .iter()
        .map(|r| (get_pk_value(r, pk_col), r))
        .collect();

    // INSERT: in new but not in old
    for (pk, new_row) in &new_by_pk {
        if !old_by_pk.contains_key(pk) {
            diffs.push(RowDiff {
                pk_column: pk_col.to_string(),
                pk_value: pk.clone(),
                change_type: "added".to_string(),
                old_values: None,
                new_values: Some((*new_row).clone()),
                changed_columns: new_row.keys().cloned().collect(),
            });
        }
    }

    // DELETE: in old but not in new
    for (pk, old_row) in &old_by_pk {
        if !new_by_pk.contains_key(pk) {
            diffs.push(RowDiff {
                pk_column: pk_col.to_string(),
                pk_value: pk.clone(),
                change_type: "removed".to_string(),
                old_values: Some((*old_row).clone()),
                new_values: None,
                changed_columns: old_row.keys().cloned().collect(),
            });
        }
    }

    // UPDATE: in both but values differ
    for (pk, new_row) in &new_by_pk {
        if let Some(old_row) = old_by_pk.get(pk) {
            let changed_cols: Vec<String> = new_row
                .iter()
                .filter(|(col, new_val)| old_row.get(*col).map(|o| o != *new_val).unwrap_or(true))
                .map(|(col, _)| col.clone())
                .collect();

            if !changed_cols.is_empty() {
                diffs.push(RowDiff {
                    pk_column: pk_col.to_string(),
                    pk_value: pk.clone(),
                    change_type: "modified".to_string(),
                    old_values: Some((*old_row).clone()),
                    new_values: Some((*new_row).clone()),
                    changed_columns: changed_cols,
                });
            }
        }
    }

    diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_row(data: &[(&str, &str)]) -> RowData {
        data.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn test_detect_insert() {
        let old_rows = vec![create_row(&[("id", "1"), ("name", "Alice")])];
        let new_rows = vec![
            create_row(&[("id", "1"), ("name", "Alice")]),
            create_row(&[("id", "2"), ("name", "Bob")]),
        ];

        let diffs = calculate_all_diffs(&old_rows, &new_rows, "id");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].change_type, "added");
        assert_eq!(diffs[0].pk_value, "2");
    }

    #[test]
    fn test_detect_delete() {
        let old_rows = vec![
            create_row(&[("id", "1"), ("name", "Alice")]),
            create_row(&[("id", "2"), ("name", "Bob")]),
        ];
        let new_rows = vec![create_row(&[("id", "1"), ("name", "Alice")])];

        let diffs = calculate_all_diffs(&old_rows, &new_rows, "id");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].change_type, "removed");
        assert_eq!(diffs[0].pk_value, "2");
    }

    #[test]
    fn test_detect_update() {
        let old_rows = vec![create_row(&[("id", "1"), ("name", "Alice")])];
        let new_rows = vec![create_row(&[("id", "1"), ("name", "Alicia")])];

        let diffs = calculate_all_diffs(&old_rows, &new_rows, "id");
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0].change_type, "modified");
        assert!(diffs[0].changed_columns.contains(&"name".to_string()));
    }
}
