use std::sync::Arc;
use anyhow::Result;
use bytes::Bytes;
use crossbeam_skiplist::SkipMap;

// This mem-table under the hood uses Crossbeam's SkipList implementation.
pub struct Memtable {
  // bytes::Byte is similar to Arc<[u8]>.
  // When you clone the Bytes, or get a slice of Bytes, the underlying data will not be copied, and
  // therefore cloning it is cheap. Instead, it simply creates a new reference to the storage area
  // and the storage area will be freed when there are no reference to that area.
  map: Arc<SkipMap<Bytes, Bytes>>,
}

impl Memtable {
  // Get the value corresponding to the given key
  pub fn get(&self, key: &[u8]) -> Option<Bytes> {
    self.map.get(key).map(|entry| entry.value().clone())
  }

  // Inserts the given key-value pair into the mem-table.
  pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
    self
      .map
      .insert(Bytes::copy_from_slice(key), Bytes::copy_from_slice(value));
    Ok(())
  }
}
