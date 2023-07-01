use itertools::Itertools;

pub type TableName = String;

pub type PrimaryColName = String;

pub type ColName = String;

pub type PrimaryValue = String;

pub type Hash = String;

pub struct TableSchema {
    pub table_name: TableName,
}

pub struct ColumnSchemata {
    pub primary_col: ColumnSchema,
    pub cols: Vec<ColumnSchema>,
}

impl ColumnSchemata {
    pub fn new(primary_col: ColumnSchema, cols: Vec<ColumnSchema>) -> Self {
        Self { primary_col, cols }
    }

    pub fn get_all_col_names(self) -> (PrimaryColName, Vec<ColName>) {
        (self.primary_col.col_name, self.cols.into_iter().map(|col| col.col_name).collect())
    }

    pub fn get_all_col_refs(&self) -> Vec<&ColumnSchema> {
        let mut cols = self.cols.iter().collect_vec();
        cols.insert(0, &self.primary_col);
        cols
    }

    pub fn count(&self) -> usize {
        self.cols.len() + 1
    }
}

#[derive(Clone)]
pub struct ColumnSchema {
    pub col_name: ColName,
    pub data_type: String,
    pub column_type: String,
}
