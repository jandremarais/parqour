use arrow::error::ArrowError;
use parquet::errors::ParquetError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),

    #[error("parquet error: `{0}`")]
    ParquetError(#[from] ParquetError),

    #[error("arrow error: `{0}`")]
    ArrowError(#[from] ArrowError),

    /// Error that may occur while receiving messages from the channel.
    #[error("Channel receive error: `{0}`")]
    ChannelReceiveError(#[from] std::sync::mpsc::RecvError),
}

pub type Result<T> = std::result::Result<T, Error>;
