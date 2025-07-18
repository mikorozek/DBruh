use std::{collections::BTreeMap, fs, path::PathBuf};

use dbruh_types::{ColumnDefinition, TableSchema};

type Key = Vec<u8>
type Value = Vec<u8>

pub struct Database {
    memtable: BTreeMap<Key, Value>,
    database_path: PathBuf,
    merge_interval: i32,
    memtable_size: i32,
}

impl Database {
    pub fn new(name: &str, merge_interval: i32, memtable_size: i32) -> Result<Self, String> {
        let path_buf = PathBuf::from(format!("/var/lib/dbruh/{}", name));
        if path_buf.exists() {
            return Err(format!("Database {} already exists.", name))
        }
        Ok(Database {
            memtable: BTreeMap::new(),
            database_path: path_buf,
            merge_interval: merge_interval,
            memtable_size: memtable_size,
        })
    }

    pub fn create_table(
        &self, 
        table_name: &str, 
        columns: Vec<ColumnDefinition>, 
        primary_key_column: String
        ) -> Result<TableSchema, String> {
        let schema_path = self.database_path.join("schemas").join(format!("{}.json", table_name));
        let table_data_path = self.database_path.join("data").join(&table_name);
        if schema_path.exists() {
            return Err(format!("Table {} already exists.", table_name))
        }
        let schema = TableSchema { 
            table_name: table_name.to_string(), 
            columns: columns, 
            primary_key_column: primary_key_column 
        }
        let schema_json = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        fs::write(schema_path.as_path(), schema_json).map_err(|e| e.to_string())?;
        fs::create_dir_all(table_data_path).map_err(|e| e.to_string())?;

        Ok(schema)
    }

}
