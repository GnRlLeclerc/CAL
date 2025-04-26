use std::io;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

pub const SOCKET: &str = "/tmp/cal-daemon.sock";

/// Write a message to a socket (via JSON serialization)
pub fn send_message<T, W>(message: &T, writable: &mut W) -> io::Result<()>
where
    T: serde::Serialize,
    W: std::io::Write,
{
    let bytes = serde_json::to_vec(message).expect("Failed to serialize message");
    let length = bytes.len() as u32;

    writable.write_all(&length.to_be_bytes())?;
    writable.write_all(&bytes)?;

    Ok(())
}

/// Write a message to a socket (via JSON serialization)
pub async fn send_message_async<T, W>(message: &T, writable: &mut W) -> io::Result<()>
where
    T: serde::Serialize,
    W: tokio::io::AsyncWrite + Unpin,
{
    let bytes = serde_json::to_vec(message).expect("Failed to serialize message");
    let length = bytes.len() as u32;

    writable.write(&length.to_be_bytes()).await?;
    writable.write(&bytes).await?;

    Ok(())
}

/// Read a message from a socket (via JSON deserialization)
pub fn read_message<T, R>(readable: &mut R) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
{
    let mut length_bytes = [0u8; 4];
    readable.read_exact(&mut length_bytes)?;

    let length = u32::from_be_bytes(length_bytes) as usize;

    let mut bytes = vec![0u8; length];
    readable.read_exact(&mut bytes)?;

    let message: T = serde_json::from_slice(&bytes).expect("Failed to deserialize message");

    Ok(message)
}

/// Read a message from a socket (via JSON deserialization)
pub async fn read_message_async<T, R>(readable: &mut R) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
    R: tokio::io::AsyncRead + Unpin,
{
    let mut length_bytes = [0u8; 4];
    readable.read(&mut length_bytes).await?;

    let length = u32::from_be_bytes(length_bytes) as usize;

    let mut bytes = vec![0u8; length];
    readable.read(&mut bytes).await?;

    let message: T = serde_json::from_slice(&bytes).expect("Failed to deserialize message");

    Ok(message)
}
