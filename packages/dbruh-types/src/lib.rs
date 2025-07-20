use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum DataType {
    Text,
    Integer,
    Boolean,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub data_type: DataType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: HashMap<String, ColumnDefinition>,
    pub primary_key_columns: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub enum DataValue {
    Text(String),
    Integer(i32),
    Boolean(bool),
    Null,
}

#[derive(Clone)]
pub struct UpdateValue {
    pub column_name: String,
    pub data_value: DataValue,
}

#[derive(Clone)]
pub enum RowOperationType {
    RowOperation(UpdateValue),
    Delete,
}

#[derive(Clone)]
pub struct RowOperation {
    pub op_type: RowOperationType,
    pub timestamp: u64,
}

#[derive(Clone, Default)]
pub struct RowState {
    pub row_snapshot: HashMap<String, DataValue>,
    pub operations: Vec<RowOperation>,
}

impl PartialEq for RowOperation {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
    fn ne(&self, other: &Self) -> bool {
        self.timestamp != other.timestamp
    }
}

impl Eq for RowOperation {}

impl PartialOrd for RowOperation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

impl Ord for RowOperation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
