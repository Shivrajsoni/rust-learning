use chrono::NaiveDateTime;
use colored::*;
use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
enum BlockchainError {
    TimeError(String),
}

#[derive(Clone, Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    signature: Option<String>,
}

#[derive(Clone, Debug)]
struct MultipleTransactions {
    transaction_table: Vec<Transaction>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "From: {} To: {} Amount: {} Fee: {}",
            self.from, self.to, self.amount, self.fee
        )
    }
}

impl fmt::Display for MultipleTransactions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (i, transaction) in self.transaction_table.iter().enumerate() {
            result.push_str(&format!("Transaction {}: {} ", i + 1, transaction));
        }
        write!(f, "{}", result)
    }
}

const DIFFICULTY: u32 = 2;
struct Block {
    index: u32,
    prev_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, prev_hash: String, data: String) -> Result<Block, BlockchainError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| BlockchainError::TimeError(format!("Time Error : {}", e)))?;

        Ok(Block {
            index,
            prev_hash,
            timestamp: timestamp.as_secs(),
            data,
            nonce: 0,
            hash: String::new(),
        })
    }

    fn calculate_hash(&self) -> String {
        let data = format!(
            "{} {} {} {} {}",
            self.index, &self.prev_hash, self.timestamp, &self.data, self.nonce
        );
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
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY as usize] == "00" {
                println!(
                    "{}",
                    format!("Block Mined with Hash {} ", self.index).green()
                );
                if iteration > 100 {
                    println!("{}", "Mining is in process ".yellow());
                    thread::sleep(Duration::from_secs(3));
                    println!("{}", format!("Mined Hash: {} ", self.hash).cyan());
                    break;
                }
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime =
            NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0).unwrap_or_default();
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    fn new() -> Result<BlockChain, BlockchainError> {
        let genesis_block = Block::new(0, String::new(), String::from("genesis block"))?;
        Ok(BlockChain {
            chain: vec![genesis_block],
        })
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
    println!("{}", "Welcome to Blockchain Simulator!".blue().bold());

    println!("{}", "Enter the Miner Name: ".yellow());
    let mut miner_name = String::new();
    std::io::stdin().read_line(&mut miner_name).unwrap();
    miner_name = miner_name.trim().to_string();

    println!("{}", "Starting the Blockchain Simulation".green());

    let trader_names = vec![
        "Shivraj", "jarvihs", "phantom", "metamask", "larry", "harry", "zain", "watson", "anna",
    ];

    let mut nexa = match BlockChain::new() {
        Ok(chain) => chain,
        Err(e) => {
            println!("{}", format!("Error Creating Blockchain : {:?}", e).red());
            return;
        }
    };

    let mut sender = miner_name.clone();

    for i in 0..trader_names.len() {
        println!("{}", format!("Mining Block: {}", i + 1).yellow());
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };

        // Create multiple transactions for each block
        let mut transactions = Vec::new();

        // First transaction
        let transaction1 = Transaction {
            from: sender.clone(),
            to: recipient.clone(),
            amount: 1000,
            fee: 10,
            signature: None,
        };
        transactions.push(transaction1);

        // Second transaction
        let transaction2 = Transaction {
            from: recipient.clone(),
            to: sender.clone(),
            amount: 2000,
            fee: 20,
            signature: None,
        };
        transactions.push(transaction2);

        // Third transaction
        let transaction3 = Transaction {
            from: sender.clone(),
            to: recipient.clone(),
            amount: 3000,
            fee: 30,
            signature: None,
        };
        transactions.push(transaction3);

        let multiple_transactions = MultipleTransactions {
            transaction_table: transactions.clone(),
        };

        let new_block = match Block::new(
            (i + 1) as u32,
            String::new(),
            multiple_transactions.to_string(),
        ) {
            Ok(block) => block,
            Err(e) => {
                println!("{}", format!("Error creating new block: {:?}", e).red());
                continue;
            }
        };
        nexa.add_new_block(new_block);

        // Display all transactions in this block
        println!("{}", format!("Block {} Transactions:", i + 1).cyan().bold());
        for (idx, transaction) in transactions.iter().enumerate() {
            println!(
                "{}",
                format!("  Transaction {}: {}", idx + 1, transaction).blue()
            );
        }
        println!();

        sender = recipient;
    }

    let total_blocks = nexa.get_total_block();
    println!(
        "{}",
        format!(
            "Total Blocks added in the Nexa Blockchain: {}",
            total_blocks
        )
        .green()
    );

    let nexa_per_block = 137;
    let nexa_traded = nexa_per_block * total_blocks;
    println!("{}", format!("Total Nexa traded: {}", nexa_traded).yellow());

    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time run backwards")
        .as_secs();
    let end_date = NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0).unwrap_or_default();
    println!("{}", format!("Simulation ended at {}", end_date).blue());
    println!(
        "{}",
        "Congratulations! You have successfully completed setting up the blockchain locally"
            .green()
            .bold()
    );
}
