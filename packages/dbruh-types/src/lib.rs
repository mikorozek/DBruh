use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum DataType {
    Text,
    Integer,
    Boolean,
    Uuid,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
    pub primary_key_column: String,
}
