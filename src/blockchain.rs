use crate::block::{Block, BlockData};

#[derive(Clone)]
pub struct BlockChain<T> {
    pub min_mine_time: u64,
    pub max_mine_time: u64,
    blocks: Vec<Block<T>>,
}

#[derive(Debug)]
pub enum VerifyResult {
    Valid,
    Invalid(usize),
}

impl<T: Default + BlockData> BlockChain<T> {
    pub fn new() -> Self {
        Self {
            blocks: vec![
                Block::<T> {
                    id: 0,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                    previous_hash: [0; 32],
                    data: T::default(),
                    nonce: 0,
                    difficulty: 0
                }
            ],
            min_mine_time: 1000,
            max_mine_time: 1000,
        }
    }
    /// returns a new unmined block that needs to be mined before it can be attached
    pub fn new_block(&self, data: T) -> Block<T> {
        // fetch the last block
        let previous_block = self.blocks.last().unwrap();

        // get the time stamp
        let time = chrono::Utc::now().timestamp_millis();

        // the time since the last block is used to decide difficulty
        let delta = (time - previous_block.timestamp).max(0) as u64;
        let difficulty = if delta < self.min_mine_time {
            previous_block.difficulty.min(u32::MAX - 1) + 1
        } else if delta > self.max_mine_time {
            previous_block.difficulty.max(u32::MIN + 1) - 1
        } else {
            previous_block.difficulty
        };

        // initialize the block struct
        Block::<T> {
            id: previous_block.id + 1,
            timestamp: time,
            previous_hash: previous_block.hash(),
            data,
            nonce: 0,
            difficulty,
        }
    }
    pub fn verify(&self) -> VerifyResult {
        // a simple function to check if the blockhain is valid
        for i in 0..(self.blocks.len() - 1) {

            // compare if the stored hash is equal to the hash of the previous block
            if self.blocks[i].hash() != self.blocks[i + 1].previous_hash {
                return VerifyResult::Invalid(i);
            }
        }
        VerifyResult::Valid
    }
    pub fn last(&self) -> &Block<T> {
        self.blocks.last().unwrap()
    }
    pub fn push(&mut self, block: Block<T>) {
        self.blocks.push(block);
    }
}
