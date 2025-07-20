use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};

use dbruh_types::{ColumnDefinition, RowOperation, RowState, TableSchema};

pub struct Database {
    memtables: HashMap<String, BTreeMap<Vec<u8>, RowState>>,
    database_path: PathBuf,
    merge_interval: i32,
    memtable_size: i32,
}

impl Database {
    pub fn new(name: &str, merge_interval: i32, memtable_size: i32) -> Result<Self, String> {
        let path_buf = PathBuf::from(format!("/var/lib/dbruh/{}", name));
        let schemas_path_buf = path_buf.join("schemas");
        let mut memtables: HashMap<String, BTreeMap<Vec<u8>, Vec<RowOperation>>> = HashMap::new();
        if schemas_path_buf.exists() {
            for entry_result in fs::read_dir(&schemas_path_buf).map_err(|e| e.to_string())? {
                if let Ok(entry) = entry_result {
                    let schema_path = entry.path();
                    if schema_path.is_file() {
                        if let Some(table_name) = schema_path.file_stem().and_then(|s| s.to_str()) {
                            memtables.insert(table_name.to_string(), BTreeMap::new());
                        }
                    }
                }
            }
        }
        Ok(Database {
            memtables: memtables,
            database_path: path_buf,
            merge_interval: merge_interval,
            memtable_size: memtable_size,
        })
    }

    fn get_table_schema(&self, table_name: &str) -> Result<TableSchema, String> {
        let schema_path = self
            .database_path
            .join("schemas")
            .join(format!("{}.json", table_name));
        let schema_text = fs::read_to_string(&schema_path)
            .map_err(|e| format!("Unable to read the file {}: {}", schema_path.display(), e))?;
        let schema: TableSchema = serde_json::from_str(&schema_text).map_err(|e| e.to_string())?;
        Ok(schema)
    }

    pub fn create_table(
        &mut self,
        table_name: &str,
        columns: HashMap<String, ColumnDefinition>,
        primary_key_columns: Vec<String>,
    ) -> Result<TableSchema, String> {
        self.memtables
            .insert(table_name.to_string(), BTreeMap::new());
        let schema_path = self
            .database_path
            .join("schemas")
            .join(format!("{}.json", table_name));
        let table_data_path = self.database_path.join("data").join(&table_name);
        if schema_path.exists() {
            return Err(format!("Table {} already exists.", table_name));
        }
        let schema = TableSchema {
            table_name: table_name.to_string(),
            columns: columns,
            primary_key_columns: primary_key_columns,
        };
        let schema_json = serde_json::to_string_pretty(&schema).map_err(|e| e.to_string())?;
        fs::write(schema_path.as_path(), schema_json).map_err(|e| e.to_string())?;
        fs::create_dir_all(table_data_path).map_err(|e| e.to_string())?;

        Ok(schema)
    }

    pub fn put(
        &mut self,
        table_name: &str,
        primary_key: &Vec<u8>,
        row_op: &RowOperation,
    ) -> Result<(), String> {
        // TODO: Add handling of multithreading
        let btree_map = match self.memtables.get_mut(table_name) {
            Some(val) => val,
            None => {
                return Err(format!(
                    "Error while trying to get memtable for {}.",
                    table_name
                ));
            }
        };
        let row: &mut RowState = btree_map.entry(primary_key.clone()).or_default();
        row.operations.push(row_op.clone());
        Ok(())
    }
}
