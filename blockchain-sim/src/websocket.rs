use crate::events::{ConnectionManager, EventBus};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use uuid::Uuid;
use warp::Filter;

// 🎯 What is a WebSocket?
// A WebSocket is like a phone call between your browser and server.
// Unlike regular web requests (like asking for a webpage), WebSockets
// stay connected and can send messages back and forth in real-time!

pub struct WebSocketServer {
    event_bus: EventBus,
    connection_manager: Arc<ConnectionManager>,
}

impl WebSocketServer {
    pub fn new(event_bus: EventBus, connection_manager: Arc<ConnectionManager>) -> Self {
        Self {
            event_bus,
            connection_manager,
        }
    }

    // Start the WebSocket server
    pub async fn start(&self, port: u16) {
        let addr = format!("127.0.0.1:{}", port);
        println!("🚀 Starting WebSocket server on ws://{}", addr);

        let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
        println!("✅ WebSocket server listening on ws://{}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            println!("📞 New connection from: {}", addr);

            // Clone the event bus and connection manager for this connection
            let event_bus = self.event_bus.clone();
            let connection_manager = Arc::clone(&self.connection_manager);

            // Handle each connection in a separate task (like a separate thread)
            tokio::spawn(async move {
                Self::handle_connection(stream, event_bus, connection_manager).await;
            });
        }
    }

    // Handle a single WebSocket connection
    async fn handle_connection(
        stream: TcpStream,
        event_bus: EventBus,
        connection_manager: Arc<ConnectionManager>,
    ) {
        // Accept the WebSocket connection
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                eprintln!("❌ Failed to accept WebSocket connection: {}", e);
                return;
            }
        };

        // Generate a unique ID for this connection
        let connection_id = Uuid::new_v4();

        // Add this connection to our manager
        connection_manager.add_connection(connection_id).await;

        // Split the WebSocket into sender and receiver
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 🎯 What are we doing here?
        // We're creating two tasks that run at the same time:
        // 1. Task 1: Listen for messages from the client
        // 2. Task 2: Send blockchain events to the client

        // Task 1: Handle incoming messages from the client
        let client_task = tokio::spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(msg) => {
                        // Handle client messages here
                        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                            println!("📨 Received from client {}: {}", connection_id, text);

                            // You can add custom commands here
                            if text == "ping" {
                                println!("Pong Pong from client {}", connection_id);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });

        // Task 2: Send blockchain events to the client
        let event_task = tokio::spawn(async move {
            // Subscribe to blockchain events
            let mut event_receiver = event_bus.subscribe();

            while let Ok(event) = event_receiver.recv().await {
                // Convert the event to JSON
                let event_json = match serde_json::to_string(&event) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("❌ Failed to serialize event: {}", e);
                        continue;
                    }
                };

                // Send the event to the client
                if let Err(e) = ws_sender
                    .send(tokio_tungstenite::tungstenite::Message::Text(event_json))
                    .await
                {
                    eprintln!("❌ Failed to send event to client: {}", e);
                    break;
                }
            }
        });

        // Wait for either task to complete
        tokio::select! {
            _ = client_task => println!("👋 Client task ended for {}", connection_id),
            _ = event_task => println!("📡 Event task ended for {}", connection_id),
        }

        // Clean up when connection ends
        connection_manager.remove_connection(connection_id).await;
    }
}

// 🎯 What are API Endpoints?
// API endpoints are like different doors to your house.
// Each door (endpoint) gives you different information.

// Create REST API endpoints using Warp
pub fn create_api_routes(
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
    connection_manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply> + Clone {
    // GET /api/blocks - Get all blocks
    let get_blocks = warp::path!("api" / "blocks")
        .and(warp::get())
        .and(with_blockchain(Arc::clone(&blockchain)))
        .and_then(get_all_blocks);

    // GET /api/blocks/{index} - Get a specific block
    let get_block = warp::path!("api" / "blocks" / u32)
        .and(warp::get())
        .and(with_blockchain(Arc::clone(&blockchain)))
        .and_then(get_block_by_index);

    // GET /api/status - Get blockchain status
    let get_status = warp::path!("api" / "status")
        .and(warp::get())
        .and(with_blockchain(Arc::clone(&blockchain)))
        .and(with_connection_manager(Arc::clone(&connection_manager)))
        .and_then(get_blockchain_status);

    // GET /api/transactions - Get all transactions
    let get_transactions = warp::path!("api" / "transactions")
        .and(warp::get())
        .and(with_blockchain(Arc::clone(&blockchain)))
        .and_then(get_all_transactions);

    // GET /api/blocks/{index}/transactions - Get transactions for a specific block
    let get_block_transactions = warp::path!("api" / "blocks" / u32 / "transactions")
        .and(warp::get())
        .and(with_blockchain(Arc::clone(&blockchain)))
        .and_then(get_block_transactions);

    // Combine all routes
    get_blocks
        .or(get_block)
        .or(get_status)
        .or(get_transactions)
        .or(get_block_transactions)
}

// Helper function to inject blockchain into route handlers
fn with_blockchain(
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
) -> impl Filter<
    Extract = (Arc<tokio::sync::RwLock<crate::BlockChain>>,),
    Error = std::convert::Infallible,
> + Clone {
    warp::any().map(move || Arc::clone(&blockchain))
}

// Helper function to inject connection manager into route handlers
fn with_connection_manager(
    connection_manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (Arc<ConnectionManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&connection_manager))
}

// API Route Handlers

async fn get_all_blocks(
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.read().await;
    Ok(warp::reply::json(&*blockchain))
}

async fn get_block_by_index(
    index: u32,
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.read().await;

    if let Some(block) = blockchain.chain.get(index as usize) {
        Ok(warp::reply::json(block))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn get_blockchain_status(
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
    connection_manager: Arc<ConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.read().await;
    let connection_count = connection_manager.connection_count().await;

    let status = json!({
        "total_blocks": blockchain.chain.len(),
        "connected_clients": connection_count,
        "last_block_hash": blockchain.chain.last().map(|b| &b.hash),
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    Ok(warp::reply::json(&status))
}

async fn get_all_transactions(
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.read().await;

    let mut all_transactions = Vec::new();

    for (block_index, block) in blockchain.chain.iter().enumerate() {
        for transaction in &block.data.transaction_table {
            all_transactions.push(json!({
                "block_index": block_index,
                "from": transaction.from,
                "to": transaction.to,
                "amount": transaction.amount,
                "fee": transaction.fee,
                "block_hash": block.hash
            }));
        }
    }

    Ok(warp::reply::json(&all_transactions))
}

async fn get_block_transactions(
    block_index: u32,
    blockchain: Arc<tokio::sync::RwLock<crate::BlockChain>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let blockchain = blockchain.read().await;

    if let Some(block) = blockchain.chain.get(block_index as usize) {
        let transactions = block
            .data
            .transaction_table
            .iter()
            .map(|transaction| {
                json!({
                    "block_index": block_index,
                    "from": transaction.from,
                    "to": transaction.to,
                    "amount": transaction.amount,
                    "fee": transaction.fee,
                    "block_hash": block.hash,
                    "signature": transaction.signature
                })
            })
            .collect::<Vec<_>>();

        Ok(warp::reply::json(&transactions))
    } else {
        Err(warp::reject::not_found())
    }
}
