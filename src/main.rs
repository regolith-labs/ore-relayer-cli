mod open_escrow;
mod open_relayer;
mod send_and_confirm;
mod utils;

use std::sync::Arc;

use clap::{command, Parser, Subcommand};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};

struct Relayer {
    pub keypair_filepath: Option<String>,
    pub rpc_client: Arc<RpcClient>,
}

#[derive(Parser, Debug)]
#[command(about, version)]
struct Args {
    #[arg(
        long,
        value_name = "NETWORK_URL",
        help = "Network address of your RPC provider",
        global = true
    )]
    rpc: Option<String>,

    #[clap(
        global = true,
        short = 'C',
        long = "config",
        id = "PATH",
        help = "Filepath to config file."
    )]
    pub config_file: Option<String>,

    #[arg(
        long,
        value_name = "KEYPAIR_FILEPATH",
        help = "Filepath to keypair to use",
        global = true
    )]
    keypair: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Open relayer account")]
    OpenRelayer(OpenRelayerArgs),
    OpenEscrow(OpenEscrowArgs),
}

#[derive(Parser, Debug)]
struct OpenRelayerArgs;

#[derive(Parser, Debug)]
struct OpenEscrowArgs;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load the config file from custom path, the default path, or use default config values
    let cli_config = if let Some(config_file) = &args.config_file {
        solana_cli_config::Config::load(config_file).unwrap_or_else(|_| {
            eprintln!("error: Could not find config file `{}`", config_file);
            std::process::exit(1);
        })
    } else if let Some(config_file) = &*solana_cli_config::CONFIG_FILE {
        solana_cli_config::Config::load(config_file).unwrap_or_default()
    } else {
        solana_cli_config::Config::default()
    };

    // Initialize client
    let cluster = args.rpc.unwrap_or(cli_config.json_rpc_url);
    let default_keypair = args.keypair.unwrap_or(cli_config.keypair_path);
    let rpc_client = RpcClient::new_with_commitment(cluster, CommitmentConfig::confirmed());
    let relayer = Arc::new(Relayer::new(Arc::new(rpc_client), Some(default_keypair)));

    // Execute user command
    match args.command {
        Commands::OpenRelayer(_) => {
            let res = relayer.open_relayer().await;
            match res {
                Ok(_) => {}
                Err(err) => {
                    println!("err: {}", err.get_transaction_error().unwrap().to_string());
                }
            }
        }
        Commands::OpenEscrow(_) => {
            let res = relayer.open_escrow().await;
            match res {
                Ok(_) => {}
                Err(err) => {
                    println!("err: {}", err.get_transaction_error().unwrap().to_string());
                }
            }
        }
    };
}

impl Relayer {
    pub fn new(rpc_client: Arc<RpcClient>, keypair_filepath: Option<String>) -> Self {
        Self {
            rpc_client,
            keypair_filepath,
        }
    }

    pub fn signer(&self) -> Keypair {
        match self.keypair_filepath.clone() {
            Some(filepath) => read_keypair_file(filepath).unwrap(),
            None => panic!("No keypair provided"),
        }
    }

    const RELAY_MINER_FILEPATH: &str = "/etc/secrets/relay-miner.json";
    pub fn miner(&self) -> Keypair {
        read_keypair_file(Self::RELAY_MINER_FILEPATH).unwrap()
    }
}
