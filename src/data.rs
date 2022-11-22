use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub size: String,
    pub offset: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkData {
    #[serde(rename = "tx_path")]
    pub tx_path: String,
    pub packing: String,
    #[serde(rename = "data_path")]
    pub data_path: String,
    pub chunk: String,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub transaction: String,
    #[arg(long)]
    pub output: std::path::PathBuf,
}
