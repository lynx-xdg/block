use crate::blockchain::BlockChain;
use log::info;
use crate::block::{Block, BlockData};

pub struct Miner<'a, T: BlockData + Default> {
    pub chain: &'a mut BlockChain<T>
}

impl<'a, T: BlockData + Default> Miner<'a, T> {
    pub fn new(chain: &'a mut BlockChain<T>) -> Miner<'a, T> {
        Miner {
            chain: chain
        }
    }
    pub fn mine(&mut self, data: T) {
        // make a block with the data
        let mut block = self.chain.new_block(data);

        // create a number to compare the hash to
        let compare = Block::<T>::difficulty_cmp(block.difficulty);

        // timing for optimisation purposes
        let start = chrono::Utc::now().timestamp_millis();

        // while the hash is 'smaller' than the compare number the hash is invalid
        // if the hash is invalid, change to nonce value to result in a new hash
        while !(block.is_valid(compare)) {
            block.nonce += 1;
        }

        // calculate the hashrate
        let end = chrono::Utc::now().timestamp_millis();
        info!("Hashrate: {}", (block.nonce as f32) * 1000. / ((end - start) as f32));

        // when the block is valid we add it to the blockchain
        self.chain.push(block);
    }
}