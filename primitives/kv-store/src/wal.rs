use std::{
  fs::{File, OpenOptions},
  mem::{self, ManuallyDrop},
  path::PathBuf,
  ptr,
};
use bytes::BytesMut;
use memmap2::{MmapMut, MmapOptions};
use crate::{entry::Entry, result::*, utils::syncDir};

pub const MAX_ENTRY_METADATA_BYTE_SIZE: usize = 21;

pub struct Options {
  // Maximum size of the WAL file.
  // Defaults to : 1 << 30 bytes (or 1 GB).
  maxFileSize: u64,
}

pub struct WAL {
  path:             PathBuf,
  file:             ManuallyDrop<File>,
  memoryMappedFile: ManuallyDrop<MmapMut>,
  size:             u32,
  writeAt:          u32,

  // For utility purposes.
  buffer: BytesMut,
}

impl WAL {
  // Open (or create if doesn't exist) a WAL.
  pub fn open(path: PathBuf, options: Options) -> Result<Self> {
    let createFile = !path.exists();

    let file = OpenOptions::new()
      .create(createFile)
      .read(true)
      .write(true)
      .open(&path)
      .map_err(|_| Error::FileOpen)?;

    // If the WAL file just got created, then we'll set it's size and sync the parent directory of
    // the WAL file with disk.
    if createFile {
      file
        .set_len(options.maxFileSize)
        .map_err(|_| Error::FileSetLen)?;

      syncDir(&path.parent().unwrap())?;
    }

    /*
      Memory Mapping :

      In a standard file I/O operation, when we issue a read(bytes) command, the OS would fetch the
      bytes (as pages, usually of size 4KB) from the file in disk, then cache the data in kernel
      space buffer and then make a copy of the cached data in the user space (application's address
      space).

      Similarly during writes, it is first updated in the application buffer, then copied to kernel
      buffer and then scheduled to be flushed to disk.

      Usually, the address space of the kernel buffer and application buffer need not be aligned
      i.e. it could be that the application buffer occupy bytes 0 to 4095 in the virtual memory and
      the kernel buffer occupy bytes 4096 to 8192 in virtual memory.

      One of the ways we can speed up the process is to avoid copying the actual pages from the
      kernel buffer to the application buffer. This can be achieved by aligning the kernel and
      application buffer in the same address space in the virtual memory.

      Memory mapping technique is used to achieve this.

      REFERENCE : https://mecha-mind.medium.com/understanding-when-and-how-to-use-memory-mapped-files-b94707df30e9.
    */
    let memoryMappedFile = unsafe {
      MmapOptions::new()
        .map_mut(&file)
        .map_err(|_| Error::FileMemoryMap)?
    };

    let mut wal = WAL {
      path,
      file: ManuallyDrop::new(file),
      size: memoryMappedFile.len() as u32,
      memoryMappedFile: ManuallyDrop::new(memoryMappedFile),
      writeAt: 0,
      buffer: BytesMut::new(),
    };

    wal.writeZerosInNextEntryMetadataMemoryRegion();

    Ok(wal)
  }
}

impl WAL {
  pub fn writeEntry(&mut self, entry: &Entry) -> Result<()> {
    self.buffer.clear();

    entry.encode(&mut self.buffer);

    self.memoryMappedFile[self.writeAt as usize..(self.writeAt as usize + self.buffer.len())]
      .clone_from_slice(&self.buffer[..]);

    self.writeAt += self.buffer.len() as u32;

    self.writeZerosInNextEntryMetadataMemoryRegion();

    Ok(())
  }

  // Write zeros in the memory region where metadata will be encoded for the next entry.
  // TODO : Explain why we need this.
  pub fn writeZerosInNextEntryMetadataMemoryRegion(&mut self) {
    let memoryRegion = &mut self.memoryMappedFile
      [self.writeAt as usize..(self.writeAt as usize + MAX_ENTRY_METADATA_BYTE_SIZE)];

    unsafe {
      ptr::write_bytes(memoryRegion.as_mut_ptr(), 0, memoryRegion.len());
    }
  }
}
