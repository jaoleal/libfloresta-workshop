//! You can see this example in https://github.com/vinteumorg/Floresta
//! inside the floresta crate.
use std::str::FromStr;
use std::sync::Arc;

use bitcoin::BlockHash;
use floresta::chain::pruned_utreexo::BlockchainInterface;
use floresta::chain::AssumeValidArg;
use floresta::chain::ChainState;
use floresta::chain::KvChainStore;
use floresta::chain::Network;
use floresta::wire::mempool::Mempool;
use floresta::wire::node::UtreexoNode;
use floresta::wire::node_interface::NodeMethods;
use floresta::wire::running_node::RunningNode;
use floresta::wire::UtreexoNodeConfig;
use rustreexo::accumulator::pollard::Pollard;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use wk_utils::cleanup;
use wk_utils::get_tempdir;

const TEMP_DATA_DIR: &str = "~/.floresta_workshop";

mod wk_utils;
#[tokio::main]
async fn main() {
    //Step 1: Create a ChainState to store the accumulator and the headers chain.

    // firstly a database
    let db =
        KvChainStore::new(TEMP_DATA_DIR.into()).expect("failed to open the blockchain database");

    // Create a new chainstate instance to keep track of our data
    let chain = Arc::new(ChainState::<KvChainStore>::new(
        db,               // Our database in $HOME/.floresta_workshop/
        Network::Bitcoin, // Network indicates which network we will store blocks from.
        // Read about [`Network`] in the bitcoin crate to know the enum.
        AssumeValidArg::Disabled, // We use AssumeValidArg for assuming when to validate blocks.
                                  // This sets to validate all blocks.
    ));

    //Step 2: Create a new node that will connect to the Bitcoin Network and start requesting blocks.
}
