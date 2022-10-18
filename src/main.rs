use base64::decode_config;
use clap::Parser;
use data::{Args, ChunkData, Root};
use std::{error::Error, fs};
extern crate base64;

mod data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    let result = get_tx_offset_data(&args.transaction).await?;

    let size = result.size.parse::<i64>()?;
    let end_offset = result.offset.parse::<i64>()?;
    let start_offset = end_offset - size + 1;

    let mut data = Vec::new();
    let mut byte = 0;

    while byte < size {
        let mut chunk_data = get_and_decode_chunk_data(start_offset + byte).await?;

        if !chunk_data.is_empty() {
            byte += chunk_data.len() as i64;
            data.append(&mut chunk_data)
        } else {
            println!("Cannot find chunk data, make sure input is correct and try again")
        }
        println!("{}%", (100 * byte / size));
    }

    fs::write(args.output, data)?;

    Ok(())
}

async fn get_tx_offset_data(tx: &str) -> Result<Root, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(format!("https://arweave.net/tx/{}/offset", tx).as_str())
        .await?
        .text()
        .await?;

    let root = serde_json::from_str::<Root>(&response)?;

    Ok(root)
}

async fn get_and_decode_chunk_data(offset: i64) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(format!("https://arweave.net/chunk/{}", offset).as_str())
        .await?
        .text()
        .await?;

    let chunkdata = serde_json::from_str::<ChunkData>(&response)?;
    let decoded = decode_config(chunkdata.chunk, base64::URL_SAFE)?;

    Ok(decoded)
}
