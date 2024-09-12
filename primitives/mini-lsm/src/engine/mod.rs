use std::sync::{atomic::AtomicUsize, Arc, RwLock};
use anyhow::{anyhow, Result};
use bytes::Bytes;
use mem_table::MemTable;
use state::StorageEngineState;

mod mem_table;
pub mod options;
mod state;

struct StorageEngineCore {
  state:                    Arc<RwLock<Arc<StorageEngineState>>>,
  currentMutableMemTableId: AtomicUsize,
}

impl StorageEngineCore {
  pub fn get(&self, key: &[u8]) -> Result<Option<Bytes>> {
    let state = self
      .state
      .read()
      .map_err(|error| anyhow!("Failed getting read lock on StorageEngineState : {}", error))?;

    Ok(state.mutableMemTable.get(key).filter(|value| {
      // The empty slice is called a delete tombstone and represents that the key is deleted.
      !value.is_empty()
    }))
  }

  pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
    let state = self
      .state
      .read()
      .map_err(|error| anyhow!("Failed getting read lock on StorageEngineState : {}", error))?;

    state.mutableMemTable.put(key, value)
  }

  pub fn delete(&self, key: &[u8]) -> Result<()> {
    self.put(key, &[])
  }

  // When the current mutable MemTable size has reached its limit, we'll make it immutable and
  // create a new mutable MemTable.
  fn freezeCurrentMemTable(&self) -> Result<()> {
    let currentMemtableId = self
      .currentMutableMemTableId
      .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    let newMutableMemTable = MemTable::new(currentMemtableId + 1);

    let mut state = self.state.write().map_err(|error| {
      anyhow!(
        "Failed getting write lock on StorageEngineState : {}",
        error
      )
    })?;

    unimplemented!()
  }
}

pub struct StorageEngine {}
