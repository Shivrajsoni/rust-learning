use sha2::{Digest, Sha256};
use std::fmt;
use std::string;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use chrono::NaiveDateTime;

struct Block {
    index: u32,
    prev_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, prev_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time Went backwards").as_secs();
        Block {
            index,
            prev_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&self) -> String {
        let data = format!("{} {} {} {} {}", self.index, &self.prev_hash, self.timestamp, &self.data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn mine_block_with_visual_hash(&mut self) {
        let mut iteration = 0;
        loop {
            self.hash = self.calculate_hash();
            iteration += 1;
            if !self.hash.is_empty() && &self.hash[..2] == "00" {
                println!("Block Mined with Hash {} ", self.index);
                if iteration > 100 {
                    println!("Mining is in process ");
                    thread::sleep(Duration::from_secs(3));
                    println!("Mined Hash: {} ", self.hash);
                    break;
                }
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0)
            .unwrap_or_default();
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        let genesis_block = Block::new(0, String::new(), String::from("genesis block"));
        BlockChain { chain: vec![genesis_block] }
    }

    fn add_new_block(&mut self, mut new_block: Block) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        new_block.prev_hash = prev_hash;
        new_block.mine_block_with_visual_hash();
        self.chain.push(new_block);
    }

    fn get_total_block(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!("Welcome to Blockchain Simulator!");

    println!("Enter the Miner Name: ");
    let mut miner_name = String::new();
    std::io::stdin().read_line(&mut miner_name).unwrap();
    miner_name = miner_name.trim().to_string();

    println!("Starting the Blockchain Simulation");

    let trader_names = vec!["Shivraj", "jarvihs", "phantom", "metamask", "larry", "harry", "zain", "watson", "anna"];

    let mut nexa = BlockChain::new();
    let mut sender = miner_name.clone();

    for i in 0..trader_names.len() {
        println!("Mining Block: {}", i + 1);
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };

        let transaction = format!("Transaction from {} to {}", sender, recipient);
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());
        nexa.add_new_block(new_block);

        println!("Transaction: {}", transaction);
        sender = recipient;
        println!();
    }

    let total_blocks = nexa.get_total_block();
    println!("Total Blocks added in the Nexa Blockchain: {}", total_blocks);
    
    let nexa_per_block = 137;
    let nexa_traded = nexa_per_block * total_blocks;
    println!("Total Nexa traded: {}", nexa_traded);

    let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time run backwards").as_secs();
    let end_date = NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0).unwrap_or_default();
    println!("Simulation ended at {}", end_date);

    println!("Congratulations! You have successfully completed setting up the blockchain locally");
}
