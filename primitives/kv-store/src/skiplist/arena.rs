use std::{
  cmp::{max, min},
  mem,
  ptr::copy_nonoverlapping,
};

const ADDRESS_ALIGNMENT_BIT_MASK: usize = 7;

pub(super) struct Arena {
  // Pointer to the beginning of the underlying buffer.
  beginning: *mut u8,

  // Size (in bytes) of the underlying buffer.
  capacity: usize,

  // Size (in bytes) of the buffer which is currently allocated by the Arena.
  currentSize: usize,
}

impl Arena {
  pub fn new(capacity: usize) -> Self {
    /*
      The Rust allocator guarantees that u64 will always have 8 byte alignment.
      REFERENCE : https://faultlore.com/blah/rust-layouts-and-abis/.

      Notice that we only allocate the memory and don't initialize it. This is because, we
      actually will be storing u8 and not u64 data in that allocated memory region.

      TODO : Explain the benefits of 8 byte memory alignment.
    */
    let mut buffer: Vec<u64> = Vec::with_capacity(capacity / 8);

    let arenaBeginning = buffer.as_mut_ptr() as *mut u8;

    // We want our Arena and not Rust's automatic memory management to be in charge of the
    // buffer. When the Arena gets dropped, this buffer will be deallocated.
    mem::forget(buffer);

    Self {
      beginning: arenaBeginning,
      capacity,
      currentSize: 0,
    }
  }

  // Allocates 8 byte alligned memory in the underlying buffer.
  pub fn allocate(&mut self, size: usize) {
    // Adjust the allocation size to be a multiple of 8, since we want the memory to be 8 byte
    // alligned.
    // For example, if we are requested to allocate 9 bytes, we'll actually allocate 16 bytes. The
    // extra 7 bytes will be padded.
    let size = (size + ADDRESS_ALIGNMENT_BIT_MASK) & !ADDRESS_ALIGNMENT_BIT_MASK;

    // If not enough space is left, we'll create and use a new bigger buffer.
    if (self.currentSize + size) > self.capacity {
      // Create a new bigger buffer.
      let growBy = max(size, min(self.capacity, 1 << 30)); // NOTE : 1 << 30 = 1GB.
      let mut newBuffer: Vec<u64> = Vec::with_capacity((self.capacity + growBy) / 8);

      let newArenaBeginning = newBuffer.as_mut_ptr() as *mut u8;

      // Copy data from the current to the new buffer.
      unsafe {
        copy_nonoverlapping(self.beginning, newArenaBeginning, self.capacity);
      }

      // Release the current underlying buffer.
      let currentArenaBeginning = self.beginning as *mut u64;
      unsafe {
        Vec::from_raw_parts(currentArenaBeginning, 0, self.capacity);
      }

      // Use the new buffer.
      self.beginning = newArenaBeginning;
      self.capacity = self.capacity + growBy;
      mem::forget(newBuffer);
    }

    self.currentSize = self.currentSize + size;
  }
}

impl Drop for Arena {
  // Deallocate the underlying buffer.
  fn drop(&mut self) {
    let arenaBeginning = self.beginning as *mut u64;
    let capacity = self.capacity / 8;
    unsafe {
      Vec::from_raw_parts(arenaBeginning, 0, capacity);
    }
  }
}
