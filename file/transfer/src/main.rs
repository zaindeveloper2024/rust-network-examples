use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Sha256, Digest};
use std::path::PathBuf;
use thiserror::Error;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, Error as IoError},
    net::{TcpListener, TcpStream},
    time::{timeout, Duration, error::Elapsed},
};

#[derive(Error, Debug)]
pub enum TransferError {
    #[error("IO error: {0}")]
    Io(#[from] IoError),
    #[error("Connection timeout")]
    Timeout(#[from] Elapsed),
    #[error("Transfer incomplete: expected {expected} bytes, received {received} bytes")]
    Incomplete {
        expected: u64,
        received: u64,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Send {
        #[arg(short, long)]
        file: PathBuf,

        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    Receive {
        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

const BUFFER_SIZE: usize = 15; // TOBE_CHANGED
const TIMEOUT_SECS: u64 = 30;

async fn handle_timeout<T, E>(
    future: impl std::future::Future<Output = Result<T, E>>,
) -> Result<T, TransferError>
where
    TransferError: From<E>,
{
    match timeout(Duration::from_secs(TIMEOUT_SECS), future).await {
        Ok(result) => Ok(result?),
        Err(elapsed) => Err(TransferError::Timeout(elapsed)),
    }
}

async fn write_u64_to_stream(stream: &mut TcpStream, n: u64) -> Result<(), TransferError> {
    let bytes = n.to_be_bytes();
    handle_timeout(stream.write_all(&bytes)).await?;
    handle_timeout(stream.flush()).await?;
    Ok(())
}

async fn read_u64_from_stream(stream: &mut TcpStream) -> Result<u64, TransferError> {
    let mut buf = [0u8; 8];
    handle_timeout(stream.read_exact(&mut buf)).await?;
    Ok(u64::from_be_bytes(buf))
}

async fn write_string_to_stream(stream: &mut TcpStream, s: &str) -> Result<(), TransferError> {
    let len = s.len() as u16;
    handle_timeout(stream.write_all(&len.to_be_bytes())).await?;
    handle_timeout(stream.write_all(s.as_bytes())).await?;
    handle_timeout(stream.flush()).await?;
    Ok(())
}

async fn read_string_from_stream(stream: &mut TcpStream) -> Result<String, TransferError> {
    let mut len_buf = [0u8; 2];
    handle_timeout(stream.read_exact(&mut len_buf)).await?;
    let len = u16::from_be_bytes(len_buf) as usize;

    let mut string_buf = vec![0u8; len];
    handle_timeout(stream.read_exact(&mut string_buf)).await?;

    Ok(String::from_utf8_lossy(&string_buf).into_owned())
}

#[tokio::main]
async fn main() -> Result<(), TransferError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send { file, host, port } => {
            send_file(file, &host, port).await?;
        }
        Commands::Receive { output, port } => {
            receive_file(output, port).await?;
        }
    }

    Ok(())
}

async fn send_file(file_path: PathBuf, host: &str, port: u16) -> Result<(), TransferError> {
    let mut file = File::open(&file_path).await?;
    let file_size = file.metadata().await?.len();

    let mut stream = TcpStream::connect(format!("{}:{}", host, port)).await?;
    println!("Connected to receiver");

    stream.set_nodelay(true)?;

    write_u64_to_stream(&mut stream, file_size).await?;

    let file_name = file_path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    write_string_to_stream(&mut stream, &file_name).await?;

    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut hasher = Sha256::new();
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut sent = 0;

    while sent < file_size {
        let n = handle_timeout(file.read(&mut buffer)).await?;
        if n == 0 { break; }
    
        handle_timeout(stream.write_all(&buffer[..n])).await?;
        handle_timeout(stream.flush()).await?;
    
        hasher.update(&buffer[..n]);
        sent += n as u64;
        pb.set_position(sent);
    }

    if sent != file_size {
        return Err(TransferError::Incomplete {
            expected: file_size,
            received: sent,
        });
    }

    let hash = hasher.finalize();
    handle_timeout(stream.write_all(&hash)).await?;
    handle_timeout(stream.flush()).await?;

    pb.finish_with_message("Transfer completed");
    println!("File hash: {}", hex::encode(hash));

    Ok(())
}

async fn receive_file(output_path: PathBuf, port: u16) -> Result<(), TransferError> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Listening on port {}", port);

    let (mut stream, addr) = listener.accept().await?;
    println!("Accepted connection from {}", addr);

    stream.set_nodelay(true)?;

    let file_size = read_u64_from_stream(&mut stream).await?;

    let file_name = read_string_from_stream(&mut stream).await?;
    println!("Receiving file: {} ({} bytes)", file_name, file_size);

    let mut file = File::create(&output_path).await?;

    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut hasher = Sha256::new();
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut received = 0;

    tokio::time::sleep(Duration::from_millis(100)).await;

    while received < file_size {
        let n = match handle_timeout(stream.read(&mut buffer)).await? {
            0 => {
                return Err(TransferError::Incomplete {
                    expected: file_size,
                    received,
                });
            }
            n => n,
        };

        handle_timeout(file.write_all(&buffer[..n])).await?;
        file.flush().await?;

        hasher.update(&buffer[..n]);
        received += n as u64;
        pb.set_position(received);
    }

    let mut received_hash = [0u8; 32];
    handle_timeout(stream.read_exact(&mut received_hash)).await?;

    let calculated_hash = hasher.finalize();

    pb.finish_with_message("Transfer completed");

    if received_hash == calculated_hash[..] {
        println!("File hash verified: {}", hex::encode(calculated_hash));
        println!("Transfer successful!");
    } else {
        println!("Warning: File hash mismatch!");
        println!("Received:    {}", hex::encode(received_hash));
        println!("Calculated:  {}", hex::encode(calculated_hash));
    }

    Ok(())
}