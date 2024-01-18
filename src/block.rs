use core::cmp::Ordering;

pub trait BlockData {
    fn to_bytes(&self) -> &[u8];
}

#[derive(Clone, Copy, Default)]
pub struct Block<T> {
    pub id: u64,
    pub previous_hash: [u8; 32],
    pub timestamp: i64,
    pub data: T,
    pub nonce: u64,
    pub difficulty: u32,
}

impl<T: BlockData> Block<T> {
    #[inline]
    pub fn hash(&self) -> [u8; 32] {
        //Sha256::digest(self.to_bytes()).into()
        blake3::hash(&self.to_bytes()).into()
    }
    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        // this is not great but I couldn't find a way to convert this struct into a &[u8] while
        // having the generic type
        let id = self.id.to_be_bytes();
        let timestamp = self.timestamp.to_be_bytes();
        let nonce = self.nonce.to_be_bytes();
        let difficulty = self.difficulty.to_be_bytes();
        let data = self.data.to_bytes();
        let mut combined = Vec::with_capacity(8 + 8 + 8 + 4 + 32 + data.len());
        combined.extend_from_slice(&id);
        combined.extend_from_slice(&timestamp);
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&difficulty);
        combined.extend_from_slice(&self.previous_hash);
        combined.extend_from_slice(data);
        combined
    }
    pub fn is_valid(&self, x: [u8; 32]) -> bool {
        // calculate the hash of the current block
        let hash = self.hash();

        // compare the hash to the cmp
        for i in 0..((self.difficulty / 256 + 1) as usize) {
            match hash[i].cmp(&x[i]) {
                Ordering::Greater => return false,
                Ordering::Less => return true,
                Ordering::Equal => continue,
            }
        }
        true
    }
    pub fn difficulty_cmp(difficulty: u32) -> [u8; 32] {
        // the max value for a u256
        let mut compare = [255u8; 32];
        let mut i = 0;
        let mut d = difficulty;
        while d != 0 {
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
