use arena::Arena;
use bytes::Bytes;

mod arena;

// Using the C memory layout ensures that fields of the struct are laid out in memory in the same
// order as they are declared in the struct. This makes the tower field to be present at the end
// of the allocated memory.
// TODO : Explain the benefit of having the tower field at the end of the memory.
#[repr(C)]
pub struct Node {
  key:   Bytes,
  value: Bytes,

  height: usize,
}

pub struct SkipList {
  arena: Arena,

  height: usize,
}

impl SkipList {
  pub fn put(&self, key: impl Into<Bytes>, value: impl Into<Bytes>) {}
}
