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

mod wk_utils;
#[tokio::main]
async fn main() {
    //Step 1: Create a ChainState to store the accumulator and the headers chain.
}
