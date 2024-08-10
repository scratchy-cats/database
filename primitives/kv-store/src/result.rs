pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Failed creating and opening file")]
  FileCreateAndOpen,

  #[error("Failed opening file")]
  FileOpen,

  #[error("Failed setting file size")]
  FileSetLen,

  #[error("Failed syncing file")]
  FileSync,

  #[error("Failed memory-mapping file")]
  FileMemoryMap,
}
