use chrono::Local;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    username: String,          // Name of the user sending the message
    content: String,           // Content of the message
    timestamp: String,         // Timestamp of when the message was sent
    message_type: MessageType, // Type of message (user or system notification)
}

// Define an enumeration for message types
#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    UserMessage,        // Represents a message from a user
    SystemNotification, // Represents system-generated messages (e.g., join/leave notifications)
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8082").await?;

    // Display server startup message with formatting
    println!("╔════════════════════════════════════════╗");
    println!("║        RETRO CHAT SERVER ACTIVE        ║");
    println!("║        Port: 8082  Host: 127.0.0.1     ║");
    println!("║        Press Ctrl+C to shutdown        ║");
    println!("╚════════════════════════════════════════╝");

    //creating a braodcast channel upto 100 connection
    let (tx, _) = broadcast::channel::<String>(100);

    loop {
        let (socket, addr) = listener.accept().await?;
        // Display connection information
        println!("┌─[{}] New connection", Local::now().format("%H:%M:%S"));
        println!("└─ Address: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_connection(socket, tx, rx).await;
        });
    }
}

async fn handle_connection(
    mut socket: TcpStream,               // TCP clinet for the stream
    tx: broadcast::Sender<String>,       // sender for incoming messages
    mut rx: broadcast::Receiver<String>, // Receiver for broadcasting messages
) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut username = String::new();

    reader.read_line(&mut username).await.unwrap();
    let username = username.trim().to_string();

    let joined_msg = ChatMessage {
        username: username.clone(),
        content: "Joined the Chat".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };

    let json_join_msg = serde_json::to_string(&joined_msg).unwrap();
    tx.send(json_join_msg).unwrap();

    let mut line = String::new();

    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result.unwrap() == 0 {
                    break;
                }

                let msg = ChatMessage {
                    username:username.clone(),
                    content:line.trim().to_string(),
                    timestamp:Local::now().format("%H:%M:%S").to_string(),
                    message_type:MessageType::UserMessage,
                };
                let json_chat_msg = serde_json::to_string(&msg).unwrap();
                tx.send(json_chat_msg).unwrap();
                line.clear();
            }

            result = rx.recv() => {
                let msg = result.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
            }
        }
    }

    let leave_msg = ChatMessage {
        username: username.clone(),
        content: "Leaving the Chat".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };

    let leave_json = serde_json::to_string(&leave_msg).unwrap();
    tx.send(leave_json).unwrap();
}
