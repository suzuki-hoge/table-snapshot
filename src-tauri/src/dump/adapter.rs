use crate::domain::schema::{ColumnSchemata, TableSchema};
use crate::domain::snapshot::RowSnapshot;

pub trait TargetDbAdapter {
    fn get_table_schemata(&mut self) -> anyhow::Result<Vec<TableSchema>>;

    fn get_col_schemata(&mut self, table_schema: &TableSchema) -> anyhow::Result<ColumnSchemata>;

    fn get_row_snapshots(&mut self, table_schema: &TableSchema, column_schemata: &ColumnSchemata) -> anyhow::Result<Vec<RowSnapshot>>;
}
