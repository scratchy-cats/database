#![allow(non_snake_case)]

mod array;

pub mod primitive_array {
  use std::fmt::Debug;
  use bitvec::vec::BitVec;
  use crate::array::{Array, ArrayBuilder};

  pub trait PrimitiveType: Copy + Send + Sync + Default + Debug + 'static {}

  impl PrimitiveType for i32 {}

  impl PrimitiveType for f32 {}

  pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    // Flattened vector of bytes that contains all the string data concatenated together.
    data:   Vec<T>,
    bitmap: BitVec,
  }

  impl<T: PrimitiveType> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn withCapacity(capacity: usize) -> Self {
      Self {
        data:   Vec::with_capacity(capacity),
        bitmap: BitVec::with_capacity(capacity),
      }
    }

    fn push(&mut self, value: Option<T>) {
      match value {
        Some(value) => {
          self.data.push(value);
          self.bitmap.push(true);
        }

        None => {
          self.data.push(T::default());
          self.bitmap.push(false);
        }
      }
    }

    fn build(self) -> Self::Array {
      PrimitiveArray {
        data:   self.data,
        bitmap: self.bitmap,
      }
    }
  }

  pub struct PrimitiveArray<T: PrimitiveType> {
    data:   Vec<T>,
    bitmap: BitVec,
  }

  impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type Builder = PrimitiveArrayBuilder<T>;

    type RefItem<'a> = T;

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>> {
      if self.bitmap[index] {
        Some(self.data[index])
      } else {
        None
      }
    }

    fn len(&self) -> usize {
      self.bitmap.len()
    }
  }

  pub type I32Array = PrimitiveArray<i32>;
  pub type F32Array = PrimitiveArray<f32>;
}

pub mod string_array {
  use std::str::from_utf8_unchecked;
  use bitvec::vec::BitVec;
  use crate::array::{Array, ArrayBuilder};

  pub struct StringArrayBuilder {
    // Flattened vector of bytes that contains all the string data concatenated together.
    data: Vec<u8>,

    // Stores the starting positions of each string in the data vector. It allows for efficient
    // retrieval of individual strings.
    offsets: Vec<usize>,

    bitmap: BitVec,
  }

  impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn withCapacity(capacity: usize) -> Self {
      let mut offsets = Vec::with_capacity(capacity + 1);
      offsets.push(0); // Offset for the first entry.

      Self {
        data: Vec::with_capacity(capacity),
        offsets,
        bitmap: BitVec::with_capacity(capacity),
      }
    }

    fn push(&mut self, value: Option<&str>) {
      match value {
        Some(value) => {
          self.data.extend(value.as_bytes());
          self.offsets.push(self.data.len()); // Offset for the next entry.
          self.bitmap.push(true);
        }

        None => {
          self.offsets.push(self.data.len()); // Offset for the next entry.
          self.bitmap.push(false);
        }
      }
    }

    fn build(self) -> Self::Array {
      StringArray {
        data:    self.data,
        bitmap:  self.bitmap,
        offsets: self.offsets,
      }
    }
  }

  pub struct StringArray {
    // Why aren't we doing : data: Vec<Option<String>> ?
    // REFER : https://github.com/amindWalker/Rust-Layout-and-Types?tab=readme-ov-file#enum.
    data:    Vec<u8>,
    offsets: Vec<usize>,
    bitmap:  BitVec,
  }

  impl Array for StringArray {
    type Builder = StringArrayBuilder;

    type RefItem<'a> = &'a str;

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>> {
      if self.bitmap[index] {
        let range = self.offsets[index]..self.offsets[index + 1];
        let value = unsafe { from_utf8_unchecked(&self.data[range]) };
        Some(value)
      } else {
        None
      }
    }

    fn len(&self) -> usize {
      self.bitmap.len()
    }
  }
}
