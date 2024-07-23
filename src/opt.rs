use clap::{Parser};

#[derive(Parser)]
#[command(
    name = "simulat0rrr", 
    version = "1.0", 
    about = "ahhhh im s1mul4t1ngggggg!!@#!", 
    long_about = None
)]

pub struct Cli {
    #[arg(long, help = "RPC URL", default_value = "https://eth.llamarpc.com")]
    pub rpc_url: String,

    #[arg(long, help = "Block Number")]
    pub block_number: Option<String>,

    #[arg(long, help = "From address")]
    pub from: Option<String>,

    #[arg(long, help = "To address", required = true)]
    pub to: String,

    #[arg(long, help = "Data (hex string)")]
    pub data: Option<String>,

    #[arg(long, help = "Value")]
    pub value: Option<String>,
}