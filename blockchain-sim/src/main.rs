use chrono::NaiveDateTime;
use colored::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// Import our new modules
mod events;
mod websocket;

use events::{BlockchainEvent, ConnectionManager, EventBus};

const DIFFICULTY: u32 = 2;

#[derive(Debug)]
enum BlockchainError {
    TimeError(String),
}

#[derive(Clone, Debug, Serialize)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    signature: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct MultipleTransactions {
    transaction_table: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
struct Block {
    index: u32,
    prev_hash: String,
    timestamp: u64,
    data: MultipleTransactions,
    nonce: u64,
    hash: String,
}

#[derive(Debug, Serialize)]
struct BlockChain {
    chain: Vec<Block>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime =
            NaiveDateTime::from_timestamp_opt(self.timestamp as i64, 0).unwrap_or_default();
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
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

impl Block {
    fn new(
        index: u32,
        prev_hash: String,
        data: MultipleTransactions,
    ) -> Result<Block, BlockchainError> {
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

    // 🎯 Updated mining function to broadcast events!
    fn mine_block_with_visual_hash(&mut self, event_bus: &EventBus, miner: &str) {
        let mut iteration = 0;

        // Broadcast that mining has started
        event_bus.broadcast(BlockchainEvent::BlockMiningStarted {
            block_index: self.index,
            miner: miner.to_string(),
            timestamp: self.timestamp,
        });

        loop {
            self.hash = self.calculate_hash();
            iteration += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY as usize] == "00" {
                println!(
                    "{}",
                    format!("Block Mined with Hash {} ", self.index).green()
                );

                // 🎯 Broadcast that block was successfully mined!
                event_bus.broadcast(BlockchainEvent::BlockMined {
                    block_index: self.index,
                    hash: self.hash.clone(),
                    miner: miner.to_string(),
                    timestamp: self.timestamp,
                    transactions_count: self.data.transaction_table.len(),
                });

                if iteration > 100 {
                    println!("{}", "Mining is in process ".yellow());
                    thread::sleep(Duration::from_secs(3));
                    println!("{}", format!("Mined Hash: {} ", self.hash).cyan());
                    break;
                }
                // Break after successful mining to avoid multiple broadcasts
                break;
            }
            self.nonce += 1;
        }
    }
}

impl BlockChain {
    fn new() -> Result<BlockChain, BlockchainError> {
        let genesis_block_data = MultipleTransactions {
            transaction_table: vec![],
        };
        let genesis_block = Block::new(0, String::new(), genesis_block_data)?;
        Ok(BlockChain {
            chain: vec![genesis_block],
        })
    }

    // 🎯 Updated to broadcast events when adding blocks
    fn add_new_block(&mut self, mut new_block: Block, event_bus: &EventBus, miner: &str) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        new_block.prev_hash = prev_hash;

        // Mine the block (this will broadcast mining events)
        new_block.mine_block_with_visual_hash(event_bus, miner);

        // Add the block to the chain
        self.chain.push(new_block);

        // 🎯 Broadcast that blockchain was updated
        event_bus.broadcast(BlockchainEvent::BlockchainUpdated {
            total_blocks: self.chain.len(),
            total_transactions: self
                .chain
                .iter()
                .map(|b| b.data.transaction_table.len())
                .sum(),
        });
    }

    fn get_total_block(&self) -> usize {
        self.chain.len()
    }
}

// 🎯 New function to create transactions (without broadcasting individual events)
fn create_transaction(
    from: &str,
    to: &str,
    amount: u64,
    fee: u64,
    _block_index: u32,
    _event_bus: &EventBus, // Keep parameter for future use but don't broadcast here
) -> Transaction {
    let transaction = Transaction {
        from: from.to_string(),
        to: to.to_string(),
        amount,
        fee,
        signature: None,
    };

    // Note: We'll broadcast all transactions together when the block is mined
    // This reduces spam and makes the events more meaningful

    transaction
}

#[tokio::main]
async fn main() {
    println!(
        "{}",
        "Welcome to Blockchain Simulator with WebSocket!"
            .blue()
            .bold()
    );

    println!("{}", "Enter the Miner Name: ".yellow());
    let mut miner_name = String::new();
    std::io::stdin().read_line(&mut miner_name).unwrap();
    miner_name = miner_name.trim().to_string();

    println!(
        "{}",
        "Starting the Blockchain Simulation with Real-time Updates".green()
    );

    // 🎯 Initialize our event system
    let event_bus = EventBus::new();
    let connection_manager = Arc::new(ConnectionManager::new());

    // Create a shared blockchain that can be accessed by multiple threads
    let blockchain = Arc::new(tokio::sync::RwLock::new(match BlockChain::new() {
        Ok(chain) => chain,
        Err(e) => {
            println!("{}", format!("Error Creating Blockchain : {:?}", e).red());
            return;
        }
    }));

    // 🎯 Start the WebSocket server in a separate task
    let ws_event_bus = event_bus.clone();
    let ws_connection_manager = Arc::clone(&connection_manager);
    tokio::spawn(async move {
        let ws_server = websocket::WebSocketServer::new(ws_event_bus, ws_connection_manager);
        ws_server.start(8080).await;
    });

    // 🎯 Start the HTTP API server in a separate task
    let api_blockchain = Arc::clone(&blockchain);
    let api_connection_manager = Arc::clone(&connection_manager);
    tokio::spawn(async move {
        let routes = websocket::create_api_routes(api_blockchain, api_connection_manager);
        println!("🌐 Starting HTTP API server on http://127.0.0.1:3000");
        warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    });

    // Give the servers a moment to start
    tokio::time::sleep(Duration::from_secs(1)).await;

    let trader_names = vec![
        "Shivraj", "jarvihs", "phantom", "metamask", "larry", "harry", "zain", "watson", "anna",
    ];

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
        let transaction1 =
            create_transaction(&sender, &recipient, 1000, 10, (i + 1) as u32, &event_bus);
        transactions.push(transaction1);

        // Second transaction
        let transaction2 =
            create_transaction(&recipient, &sender, 2000, 20, (i + 1) as u32, &event_bus);
        transactions.push(transaction2);

        // Third transaction
        let transaction3 =
            create_transaction(&sender, &recipient, 3000, 30, (i + 1) as u32, &event_bus);
        transactions.push(transaction3);

        let multiple_transactions = MultipleTransactions {
            transaction_table: transactions.clone(),
        };

        let new_block = match Block::new((i + 1) as u32, String::new(), multiple_transactions) {
            Ok(block) => block,
            Err(e) => {
                println!("{}", format!("Error creating new block: {:?}", e).red());
                continue;
            }
        };

        // 🎯 Broadcast all transactions in this block
        for (_idx, transaction) in transactions.iter().enumerate() {
            event_bus.broadcast(BlockchainEvent::TransactionCreated {
                from: transaction.from.clone(),
                to: transaction.to.clone(),
                amount: transaction.amount,
                fee: transaction.fee,
                block_index: (i + 1) as u32,
            });
        }

        // 🎯 Add the block to our shared blockchain
        {
            let mut blockchain_guard = blockchain.write().await;
            blockchain_guard.add_new_block(new_block, &event_bus, &miner_name);
        }

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

        // Small delay to see the real-time updates
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    let total_blocks = {
        let blockchain_guard = blockchain.read().await;
        blockchain_guard.get_total_block()
    };

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
        "Congratulations! You have successfully completed setting up the blockchain with WebSocket!"
            .green()
            .bold()
    );

    // Save blockchain to JSON file
    let blockchain_guard = blockchain.read().await;
    let json = serde_json::to_string_pretty(&*blockchain_guard).unwrap();
    let mut file = File::create("blockchain_data.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    println!("{} ", "Blockchain saved to the blockchain_data.json file ");

    // 🎯 Keep the servers running
    println!("🌐 WebSocket server running on ws://127.0.0.1:8080");
    println!("🌐 HTTP API server running on http://127.0.0.1:3000");
    println!("Press Ctrl+C to stop the servers");

    // Keep the main thread alive
    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
