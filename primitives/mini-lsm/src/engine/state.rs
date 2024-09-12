use std::sync::Arc;
use super::mem_table::MemTable;

pub struct StorageEngineState {
  // The current mutable mem-table, where upcoming key-value pairs will be stored.
  pub(super) mutableMemTable: Arc<MemTable>,

  // A memtable usually has a size limit (i.e., 256MB), and it will be frozen to an immutable
  // memtable when it reaches the size limit.
  pub(super) immutableMemTables: Vec<Arc<MemTable>>,
}

impl StorageEngineState {
  pub fn new() -> Self {
    Self {
      mutableMemTable:    Arc::new(MemTable::new(0)),
      immutableMemTables: vec![],
    }
  }
}
