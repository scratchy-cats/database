use bytes::{Bytes, BytesMut};

pub struct Entry {
  pub key:       Bytes,
  pub value:     Bytes,
  pub expiresAt: u64,
}

impl Entry {
  // Encodes the entry to the given buffer.
  // Returns the buffer length after the entry has been encoded.
  pub fn encode(&self, buffer: &mut BytesMut) -> usize {
    // Encode entry metadata.
    let entryMetadata = EntryMetadata {
      keyByteLen:   self.key.len() as u32,
      valueByteLen: self.value.len() as u32,
      expiresAt:    self.expiresAt,
    };
    entryMetadata.encode(buffer);

    // Encode key and value.
    buffer.extend_from_slice(&self.key);
    buffer.extend_from_slice(&self.value);

    buffer.len()
  }
}

pub struct EntryMetadata {
  pub keyByteLen:   u32,
  pub valueByteLen: u32,
  pub expiresAt:    u64,
}

impl EntryMetadata {
  pub fn encode(&self, buffer: &mut BytesMut) {
    unimplemented!()
  }
}
