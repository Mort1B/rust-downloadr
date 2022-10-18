use base64::decode_config;
use clap::Parser;
use data::{Args, ChunkData, Root};
use std::{error::Error, fs};
extern crate base64;

mod data;

// There are two ways of doing this, either(1) by starting at the size/ length of the vector we are given from calling the
// get_and_decode_chunk_data function and subtracting the length/size of each vector of bytes until we reach end_offset - size.
// The other way(2) is by subtracting the size at the beginning and adding the length/size of each vector of bytes until we reach the
// end_offset.
// By commenting *in* line 22, 25, 40-49 and commenting *out* line 27-39 you can run the code that does it the second way(2).
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();

    let result = get_tx_offset_data(&args.transaction).await?;

    let size = result.size.parse::<i64>()?;
    let end_offset = result.offset.parse::<i64>()?;
    // let start_offset = end_offset - size + 1;

    let mut data = Vec::new();
    // let mut byte = 0;

    let mut chunk = end_offset;

    while chunk > end_offset - size {
        let mut chunk_data = get_and_decode_chunk_data(chunk).await?;

        if !chunk_data.is_empty() {
            chunk -= chunk_data.len() as i64;
            data.append(&mut chunk_data)
        } else {
            println!("Cannot find chunk data, make sure input is correct and try again")
        }
    }

    // while byte < size {
    //     let mut chunk_data = get_and_decode_chunk_data(start_offset + byte).await?;
    //     if !chunk_data.is_empty() {
    //         byte += chunk_data.len() as i64;
    //         data.append(&mut chunk_data)
    //     } else {
    //         println!("Cannot find chunk data, make sure input is correct and try again")
    //     }
    //     println!("{}%", (100 * byte / size));
    // }

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
