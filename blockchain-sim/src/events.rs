use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

// 🎯 What are Events?
// Events are like messages that tell us "something happened!"
// Think of it like a notification system - when a new block is mined,
// we send a notification to everyone who's listening.

#[derive(Debug, Clone, Serialize)]
pub enum BlockchainEvent {
    // When a new block is being mined
    BlockMiningStarted {
        block_index: u32,
        miner: String,
        timestamp: u64,
    },
    // When a block is successfully mined
    BlockMined {
        block_index: u32,
        hash: String,
        miner: String,
        timestamp: u64,
        transactions_count: usize,
    },
    // When a new transaction is created
    TransactionCreated {
        from: String,
        to: String,
        amount: u64,
        fee: u64,
        block_index: u32,
    },
    // When the blockchain is updated
    BlockchainUpdated {
        total_blocks: usize,
        total_transactions: usize,
    },
}

// 🎯 What is a Broadcast Channel?
// Think of it like a radio station - one person (the broadcaster) sends messages,
// and many people (listeners) can receive those messages at the same time.

pub type EventSender = broadcast::Sender<BlockchainEvent>;
pub type EventReceiver = broadcast::Receiver<BlockchainEvent>;

// 🎯 What is a Connection Manager?
// This keeps track of all the people (clients) who are connected to our WebSocket.
// Like a guest list at a party!

#[derive(Debug)]
pub struct ConnectionManager {
    connections: Arc<tokio::sync::RwLock<HashMap<Uuid, ()>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    // Add a new client connection
    pub async fn add_connection(&self, id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.insert(id, ());
        println!("🟢 New client connected: {}", id);
    }

    // Remove a client connection
    pub async fn remove_connection(&self, id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&id);
        println!("🔴 Client disconnected: {}", id);
    }

    // Get the number of connected clients
    pub async fn connection_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }
}

// 🎯 What is an Event Bus?
// This is like the central post office that delivers all our messages.
// When something happens in the blockchain, we send it here,
// and it gets delivered to all connected clients.

#[derive(Debug, Clone)]
pub struct EventBus {
    pub sender: EventSender,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100); // Can hold 100 messages
        Self { sender }
    }

    // Send an event to all connected clients
    pub fn broadcast(&self, event: BlockchainEvent) {
        // Check if there are any active receivers before broadcasting
        let receiver_count = self.sender.receiver_count();

        if receiver_count == 0 {
            // No clients connected, just log the event without broadcasting
            println!("📝 Event occurred but no clients connected: {:?}", event);
            return;
        }

        match self.sender.send(event) {
            Ok(_) => {
                println!("📡 Broadcasting event to {} clients", receiver_count);
            }
            Err(e) => {
                // Only log as error if it's not a "no receivers" error
                if e.to_string().contains("no receivers") {
                    println!("📝 Event occurred but no clients connected");
                } else {
                    eprintln!("❌ Failed to broadcast event: {}", e);
                }
            }
        }
    }

    // Get a receiver to listen for events
    pub fn subscribe(&self) -> EventReceiver {
        self.sender.subscribe()
    }
}
