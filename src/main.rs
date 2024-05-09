
mod block;
mod blockchain;
mod miner;
mod threaded_miner;

#[derive(Default, Clone)]
pub struct Data {
    pub test: String,
}
impl block::BlockData for Data {
    fn to_bytes(&self) -> &[u8] {
        self.test.as_ref()
    }
}

/*
fn main() {
    // for logging
    simple_logger::SimpleLogger::new().init().unwrap();

    // create the blockchain struct for the type Data
    info!("Creating blockchain");
    let mut app = blockchain::BlockChain::<Data>::new();
    let block = app.new_block(Data { test: "Eeer".into() });
    println!("{:?}", block.to_bytes());
    let mut miner = miner::Miner::new(&mut app);
    
    // mine 1000 blocks to test the mining speed and functionality
    info!("Started mining 750 blocks for testing");
    for _ in 0..750 {
        let start = chrono::Utc::now().timestamp_millis();

        // mine a block with data "Eeeey"
        miner.mine(Data {
            test: "Eeey".into(),
        });

        let end = chrono::Utc::now().timestamp_millis();
        info!(
            "Block {} mined, mine rate: {:.4} Blocks/s ({}), difficulty: {}",
            miner.chain.last().id,
            1000. / ((end - start) as f32),
            end - start,
            miner.chain.last().difficulty
        );
    }
    
    // check if there were any errors with the blockchain
    info!("Verifying blockchain");
    println!("{:?}", app.verify());
}
*/

fn main() {
    let app = blockchain::BlockChain::<Data>::new();
    for _i in 0..1000 {
        let mut block = app.new_block(Data { test: "This is a test".into() });
        block.timestamp = 0;
    }
}