//! You can see this example in https://github.com/vinteumorg/Floresta
//! inside the floresta crate.

use std::sync::Arc;

use floresta::chain::pruned_utreexo::BlockchainInterface;
use floresta::chain::AssumeValidArg;

use floresta::chain::ChainState;
use floresta::chain::KvChainStore;
use floresta::chain::Network;

use floresta::wire::mempool::Mempool;
use floresta::wire::node::UtreexoNode;

use floresta::wire::running_node::RunningNode;
use floresta::wire::UtreexoNodeConfig;
use rustreexo::accumulator::pollard::Pollard;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use wk_utils::cleanup;

const TEMP_DATA_DIR: &str = ".floresta_workshop";

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

    // Now if you want to jump the IBD process.
    // You can use [`AssumeValidArg::Hardcoded`] instead of [`AssumeValidArg::Hardcoded`]

    //Step 2: Create a new node that will connect to the Bitcoin Network and start requesting blocks.
    let config = UtreexoNodeConfig::default();
    //Notes RunningNode, floresta-wire uses different node context to go to these steps:
    //  1. knowing which chain to download with chain_selector.rs
    //  2. syncing the chain, valiting and downloading blocks with sync_node.rs
    //  3. after the syncing phase we need to start listening up for new blocks and
    // handling user requests with running_node.rs
    //
    // We only need to specify running node here because it will switch
    // between contexts as needed.
    let p2p: UtreexoNode<Arc<ChainState<KvChainStore>>, RunningNode> = UtreexoNode::new(
        config,
        chain.clone(),
        Arc::new(Mutex::new(Mempool::new(Pollard::default(), 1000))),
        None,
    )
    .unwrap();

    //Step 3: Get a stop signal and Starts the node!

    // Once this is changed to true the node will do its procedure to stop.
    let mut stop_signal = false;

    // We need to set a channel to receive a signal if the node breaks.
    let (sender, receiver) = futures::channel::oneshot::channel();
    // Get a handle of the node the be able to use the methods of [`Node_Interface`].
    let handle = &p2p.get_handle();

    tokio::spawn(p2p.run(Arc::new(RwLock::new(stop_signal)), sender));
    
    //Step 4: Starts the main loop and starts consuming the node.

    loop {
        // Stop when it finish the Initial Block Download.
        if !chain.is_in_idb() {
            //The node will run until the process exit.
            //but you can also turn the stop_signal to true.
            stop_signal = true;
            break;
        }

        //Since our chain is being used and updated by the node,
        //we can keep our eyes on whats going on in the current data!
        let best_block = chain.get_best_block().unwrap();

        println!(
            "best block: heigth {:?} hash {:?}",
            best_block.0, best_block.1
        );

        // Sleep for 1 seconds, and run the loop again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    // Remove the created files.
    cleanup();
}
