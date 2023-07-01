use crate::domain::schema::TableName;
use crate::domain::snapshot::SnapshotId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TableSummaryView {
    snapshot_id: SnapshotId,
    table_name: TableName,
}

#[tauri::command]
pub fn tmp() -> TableSummaryView {
    TableSummaryView { snapshot_id: "1234".to_string(), table_name: "users".to_string() }
}
