# 🚀 Blockchain Simulator with Real-time WebSocket

A Rust-based blockchain simulator with real-time WebSocket updates and REST API endpoints for learning purposes.

## 🎯 What This Project Teaches You

### 1. **WebSocket Communication**

- Real-time bidirectional communication between server and clients
- Event-driven architecture
- Broadcasting messages to multiple connected clients

### 2. **REST API Design**

- HTTP endpoints for data querying
- JSON responses
- Stateless API design

### 3. **Event System**

- Event-driven programming
- Message broadcasting
- Real-time notifications

### 4. **Concurrent Programming**

- Async/await patterns
- Multi-threaded applications
- Shared state management

## 🏗️ Architecture Overview

```
┌─────────────────┐    WebSocket    ┌─────────────────┐
│   Web Client    │ ◄──────────────► │  Rust Server    │
│   (Browser)     │                 │                 │
└─────────────────┘                 │  ┌─────────────┐ │
                                    │  │ Event Bus   │ │
┌─────────────────┐    HTTP API     │  └─────────────┘ │
│   API Client    │ ◄──────────────► │                 │
│   (curl, etc.)  │                 │  ┌─────────────┐ │
└─────────────────┘                 │  │ Blockchain  │ │
                                    │  │ Simulator   │ │
                                    │  └─────────────┘ │
                                    └─────────────────┘
```

## 🚀 Quick Start

### 1. Install Dependencies

```bash
cargo build
```

### 2. Run the Server

```bash
cargo run
```

### 3. Open the Web Dashboard

Open `index.html` in your browser or serve it with a simple HTTP server:

```bash
python3 -m http.server 8000
# Then visit http://localhost:8000
```

## 📡 Available Endpoints

### WebSocket (Real-time Events)

- **URL**: `ws://127.0.0.1:8080`
- **Purpose**: Real-time blockchain events
- **Events**:
  - `BlockMiningStarted`: When mining begins
  - `BlockMined`: When a block is successfully mined
  - `TransactionCreated`: When a new transaction is created
  - `BlockchainUpdated`: When the blockchain is updated

### HTTP API Endpoints

- **Base URL**: `http://127.0.0.1:3000`

#### GET `/api/blocks`

Get all blocks in the blockchain

```bash
curl http://127.0.0.1:3000/api/blocks
```

#### GET `/api/blocks/{index}`

Get a specific block by index

```bash
curl http://127.0.0.1:3000/api/blocks/1
```

#### GET `/api/transactions`

Get all transactions across all blocks

```bash
curl http://127.0.0.1:3000/api/transactions
```

#### GET `/api/status`

Get blockchain status and statistics

```bash
curl http://127.0.0.1:3000/api/status
```

## 🎮 How to Use

### 1. **Start the Simulation**

Run the Rust server and enter a miner name when prompted.

### 2. **Watch Real-time Events**

Open the web dashboard to see:

- Real-time mining progress
- Transaction creation
- Block completion
- Connection status

### 3. **Test API Endpoints**

Use the buttons in the web dashboard or curl commands to test the API.

### 4. **Monitor Multiple Clients**

Open multiple browser tabs to see how multiple clients receive the same real-time updates.

## 🧠 Learning Concepts Explained

### **What is WebSocket?**

Think of WebSocket like a phone call between your browser and server:

- **Regular HTTP**: Like sending letters back and forth
- **WebSocket**: Like a phone call that stays connected
- **Real-time**: Messages can be sent instantly in both directions

### **What are Events?**

Events are like notifications that tell us "something happened!":

- When a new block is mined → Send a notification
- When a transaction is created → Send a notification
- When mining starts → Send a notification

### **What is Broadcasting?**

Broadcasting is like a radio station:

- One sender (the server)
- Many listeners (connected clients)
- Everyone gets the same message at the same time

### **What are API Endpoints?**

API endpoints are like different doors to your house:

- `/api/blocks` → Door to get all blocks
- `/api/transactions` → Door to get all transactions
- `/api/status` → Door to get current status

## 🔧 Project Structure

```
blockchain-sim/
├── src/
│   ├── main.rs          # Main blockchain logic + server startup
│   ├── events.rs        # Event system and WebSocket management
│   └── websocket.rs     # WebSocket server and API endpoints
├── index.html           # Web dashboard for real-time monitoring
├── Cargo.toml           # Rust dependencies
└── README.md           # This file
```

## 🎯 Key Features

### **Real-time Updates**

- See blocks being mined in real-time
- Watch transactions being created
- Monitor blockchain growth live

### **Multi-client Support**

- Multiple browsers can connect simultaneously
- All clients receive the same updates
- Connection management and cleanup

### **REST API**

- Query blockchain data via HTTP
- JSON responses
- Easy integration with other tools

### **Event-driven Architecture**

- Clean separation of concerns
- Scalable event system
- Easy to add new event types

## 🚀 Next Steps for Learning

1. **Add New Event Types**: Create new blockchain events
2. **Implement Authentication**: Add user authentication to WebSocket connections
3. **Add Database**: Store blockchain data in a database
4. **Create Mobile App**: Build a mobile app that connects to the WebSocket
5. **Add More API Endpoints**: Create endpoints for specific queries
6. **Implement CORS**: Handle cross-origin requests properly
7. **Add Error Handling**: Improve error handling and recovery

## 🐛 Troubleshooting

### WebSocket Connection Issues

- Make sure the Rust server is running
- Check that port 8080 is not blocked
- Verify the WebSocket URL in the browser console

### API Connection Issues

- Ensure the HTTP server is running on port 3000
- Check for CORS issues in browser console
- Verify the API endpoints are accessible

### Build Issues

- Make sure all dependencies are installed: `cargo build`
- Check Rust version: `rustc --version`
- Update dependencies if needed: `cargo update`

## 📚 Learning Resources

- [Rust Async Programming](https://rust-lang.github.io/async-book/)
- [WebSocket Protocol](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API)
- [REST API Design](https://restfulapi.net/)
- [Event-driven Architecture](https://en.wikipedia.org/wiki/Event-driven_architecture)

---

**Happy Learning! 🎉**

This project demonstrates real-world concepts used in modern blockchain applications, cryptocurrency exchanges, and real-time financial systems.
