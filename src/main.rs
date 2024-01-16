use log::info;
mod blockchain;

#[derive(serde::Serialize, Debug, Default)]
pub struct Data {
    pub test: String,
}
fn main() {
    // for logging
    simple_logger::SimpleLogger::new().init().unwrap();

    // create the blockchain struct for the type Data
    info!("Creating blockchain");
    let mut app = blockchain::BlockChain::<Data>::new();

    // mine 1000 blocks to test the mining speed and functionality
    info!("Started mining 1000 blocks for testing");
    for _ in 0..1_000 {
        info!("Start mining block");
        let start = chrono::Utc::now().timestamp_millis();

        // mine a block with data "Eeeey"
        app.mine(Data {
            test: "Eeey".to_string(),
        });

        let end = chrono::Utc::now().timestamp_millis();
        info!(
            "Block {} mined, mine rate: {:.4} Blocks/s, diffuculty: {}",
            app.blocks.last().unwrap().id,
            1000. / ((end - start) as f32),
            app.blocks.last().unwrap().difficulty
        );
    }

    // check if there were any errors with the blockchain
    info!("Verifying blockchain");
    println!("{:?}", app.verify());
}
