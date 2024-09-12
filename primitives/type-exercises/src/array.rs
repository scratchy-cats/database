use std::fmt::Debug;
use iterator::ArrayIterator;

// NOTE : `Array: 'static` here means that any type implementing this Array must have a static
// lifetime.
pub trait Array: Send + Sync + Sized + 'static {
  type Builder: ArrayBuilder<Array = Self>;

  // Type of an item in this array.
  // REFER : https://rust-lang.github.io/generic-associated-types-initiative/index.html.
  type RefItem<'a>: Copy + Debug;

  fn get(&self, index: usize) -> Option<Self::RefItem<'_>>;

  fn len(&self) -> usize;

  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  fn iter(&self) -> ArrayIterator<Self> {
    ArrayIterator::new(&self)
  }
}

pub trait ArrayBuilder {
  type Array: Array<Builder = Self>;

  fn withCapacity(capacity: usize) -> Self;

  // Appends a value to thr builder.
  // REFER : https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading
  fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

  fn build(self) -> Self::Array;
}

pub mod iterator {
  use super::Array;

  pub struct ArrayIterator<'a, A: Array> {
    array:           &'a A,
    currentPosition: usize,
  }

  impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
      if self.currentPosition >= self.array.len() {
        None
      } else {
        let item = self.array.get(self.currentPosition);
        self.currentPosition += 1;
        Some(item)
      }
    }
  }

  impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
      Self {
        array,
        currentPosition: 0,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::primitive_array::I32Array;
  use super::*;

  // Example of using generics over Array.

  fn buildArrayFromVec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
    let mut arrayBuilder = A::Builder::withCapacity(items.len());
    for item in items {
      arrayBuilder.push(*item);
    }
    arrayBuilder.build()
  }

  #[test]
  fn testBuildingI32ArrayFromVec() {
    let items = vec![Some(1), Some(2), Some(3), None, Some(5)];
    let _: I32Array = buildArrayFromVec(&items);
  }
}
