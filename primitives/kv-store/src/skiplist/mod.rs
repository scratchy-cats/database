use bytes::Bytes;

mod arena;

pub struct SkipList {}

impl SkipList {
  pub fn put(&self, key: impl Into<Bytes>, value: impl Into<Bytes>) {}
}
