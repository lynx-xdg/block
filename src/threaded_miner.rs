use std::{sync::{Mutex, Arc}, thread};

use crate::blockchain::BlockChain;
use crate::block::{Block, BlockData};

pub struct Miner<'a, T: BlockData + Default + Clone> {
    pub chain: &'a mut BlockChain<T>,
}

impl<'a, T: BlockData + Default + Clone + 'static> Miner<'a, T> {
    pub fn new(chain: &'a mut BlockChain<T>) -> Miner<'a, T> {
        Miner { chain }
    }
    pub fn mine(&mut self, data: T, num_threads: u64)
    where
        T: Send + Sync + 'static + Clone,
    {
        let block = self.chain.new_block(data);
        let chunk_size = u64::MAX / num_threads;
        let mining = Arc::new(Mutex::new(true));
        let mut handles = vec![];
        for i in 0..num_threads {
            let mut block_clone = block.clone();
            let mining_clone = Arc::clone(&mining);
            handles.push(thread::spawn(move || {
                let compare = Block::<T>::difficulty_cmp(block_clone.difficulty);
                block_clone.nonce = i * chunk_size;
                while !block_clone.is_valid(compare) {
                    block_clone.nonce += 1;
                    if (block_clone.nonce % 100 == 0) && !*mining_clone.lock().unwrap() {
                        return None;
                    }
                }
                Some(block_clone)
            }));
        }
        self.chain.push(handles.into_iter().find_map(|handle| handle.join().ok()).unwrap().unwrap());
    }
}