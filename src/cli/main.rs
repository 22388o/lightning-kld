mod client;

use crate::client::Api;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::to_string_pretty;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP address or hostname of the target machine.
    #[arg(short, long)]
    target: String,
    /// Path to the TLS cert of the target API.
    #[arg(short, long)]
    cert_path: String,
    /// Path to the macaroon for authenticating with the API.
    #[arg(short, long)]
    macaroon_path: String,
    /// Command to run.
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Fetch information about this lightning node.
    GetInfo,
    /// Fetch confirmed and unconfirmed on-chain balance.
    GetBalance,
    /// Generates new on-chain address for receiving funds.
    NewAddress,
    /// Send on-chain funds out of the wallet.
    Withdraw {
        /// The address to withdraw to.
        #[arg(long)]
        address: String,
        /// The amount to withdraw (in Satoshis). The string "all" will empty the wallet.
        #[arg(long)]
        satoshis: String,
    },
    /// Fetch a list of this nodes peers.
    ListPeers,
    /// Connect with a network peer.
    ConnectPeer {
        /// The public key (id) of the node to connect to. Optionally provide host and port [id@host:port].
        #[arg(long)]
        public_key: String,
    },
    /// Disconnect from a network peer.
    DisconnectPeer {
        /// The public key of the node to disconnect from.
        #[arg(long)]
        public_key: String,
    },
    /// Fetch a list of this nodes open channels.
    ListChannels,
    /// Open a channel with another node.
    OpenChannel {
        /// The public key of the node to open a channel with.
        #[arg(long)]
        public_key: String,
        /// Amount of satoshis to commit to the channel.
        #[arg(long)]
        satoshis: String,
        /// The number of satoshis to push to the other node side of the channel.
        #[arg(long)]
        push_msat: Option<String>,
    },
    /// Set channel fees.
    SetChannelFee {
        /// Channel ID, short channel ID or "all" for all channels.
        #[arg(long)]
        id: String,
        /// Optional value in msats added as base fee to any routed payment.
        #[arg(long)]
        base_fee: Option<u32>,
        /// Optional value that is added proportionally per-millionths to any routed payment volume in satoshi
        #[arg(long)]
        ppm_fee: Option<u32>,
    },
    /// Close a channel.
    CloseChannel {
        /// Channel ID or short channel ID to close.
        #[arg(long)]
        id: String,
    },
    /// Get node information from the network graph.
    ListNodes {
        /// Provide Node ID to get info about a single node.
        #[arg(long)]
        id: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match run_command(args) {
        Ok(_) => (),
        Err(e) => println!("Error executing command: {e}"),
    }
}

fn run_command(args: Args) -> Result<()> {
    let api = Api::new(&args.target, &args.cert_path, &args.macaroon_path)?;

    let result = match args.command {
        Command::GetInfo => to_string_pretty(&api.get_info()?)?,
        Command::GetBalance => to_string_pretty(&api.get_balance()?)?,
        Command::NewAddress => to_string_pretty(&api.new_address()?)?,
        Command::Withdraw { address, satoshis } => {
            to_string_pretty(&api.withdraw(address, satoshis)?)?
        }
        Command::ListChannels => to_string_pretty(&api.list_channels()?)?,
        Command::ListPeers => to_string_pretty(&api.list_peers()?)?,
        Command::ConnectPeer { public_key } => to_string_pretty(&api.connect_peer(public_key)?)?,
        Command::DisconnectPeer { public_key } => {
            to_string_pretty(&api.disconnect_peer(public_key)?)?
        }
        Command::OpenChannel {
            public_key,
            satoshis,
            push_msat,
        } => to_string_pretty(&api.open_channel(public_key, satoshis, push_msat)?)?,
        Command::SetChannelFee {
            id,
            base_fee,
            ppm_fee,
        } => to_string_pretty(&api.set_channel_fee(id, base_fee, ppm_fee)?)?,
        Command::CloseChannel { id } => to_string_pretty(&api.close_channel(id)?)?,
        Command::ListNodes { id } => to_string_pretty(&api.list_nodes(id)?)?,
    };
    println!("{result}");
    Ok(())
}
