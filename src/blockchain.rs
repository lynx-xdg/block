use core::fmt::Debug;
use log::info;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct BlockChain<T: Debug> {
    pub min_mine_time: u64,
    pub max_mine_time: u64,
    pub blocks: Vec<Block<T>>,
}

#[derive(serde::Serialize, Debug)]
pub struct Block<T> {
    pub id: u64,
    pub previous_hash: [u8; 32],
    pub timestamp: i64,
    pub data: T,
    pub nonce: u64,
    pub difficulty: u32,
}


#[derive(Debug)]
pub enum VerifyResult {
    Valid,
    Invalid(usize),
}

impl<T: serde::Serialize + Debug + Default> BlockChain<T> {
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
            min_mine_time: 500,
            max_mine_time: 2000,
        }
    }
    fn make_block(&self, data: T) -> Block<T> {
        // fetch the last block
        let previous_block = self.blocks.last().unwrap();

        // get the time stamp
        let time = chrono::Utc::now().timestamp_millis();

        // the time since the last block is used to decide difficulty
        // increase the difficulty when the time it takes to mine a block gets below 1000ms
        // decrease the difficulty when the time it takes to mine a block gets above 2000ms
        let delta = (time - previous_block.timestamp).max(0) as u64;
        let difficulty = if delta < self.min_mine_time {
            previous_block.difficulty + 1
        } else if delta > self.max_mine_time {
            previous_block.difficulty - 1
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
    pub fn mine(&mut self, data: T) {
        // make a block with the data
        let mut block = self.make_block(data);

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
        self.blocks.push(block);
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
}

impl<T: serde::Serialize> Block<T> {
    fn hash(&self) -> [u8; 32] {
        Sha256::digest(self.to_bytes()).into()
    }
    fn to_bytes(&self) -> Vec<u8> {
        // this is not great but I couldn't find a way to convert this struct into a &[u8] while
        // having the generic type
        bincode::serialize(self).unwrap()
    }
    fn is_valid(&self, x: [u8; 32]) -> bool {
        // calculate the hash of the current block
        let hash = self.hash();

        // compare the hash to the cmp
        for i in 0..((self.difficulty / 256 + 1) as usize) {
            match hash[i].cmp(&x[i]) {
                Ordering::Greater => return false,
                Ordering::Less    => return true,
                Ordering::Equal   => continue
            }
        }
        true
    }
    fn difficulty_cmp(difficulty: u32) -> [u8; 32] {
        // the max value for a u256
        let mut compare = [255u8; 32];
        let mut i = 0;
        let mut d = difficulty;
        while d >= 255 {
            if d >= 255 {
                compare[i] = 0;
                i += 1;
                d -= 255;
            } else {
                compare[i] -= d as u8;
                d = 0;
            }
        }
        compare
    }
}
