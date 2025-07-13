# 🔗 Blockchain Simulator API Documentation

## 🌐 Base URL

```
http://127.0.0.1:3000
```

## 📡 Available Endpoints

### 1. **GET /api/status**

Get blockchain status and statistics.

**Response:**

```json
{
  "total_blocks": 10,
  "connected_clients": 0,
  "last_block_hash": "00a79f543657f7dd31ee4afd8a5f25ed9dda4a982a6c00f065176dd21c87d5f5",
  "timestamp": 1752402297
}
```

**Test:**

```bash
curl http://127.0.0.1:3000/api/status | jq '.'
```

---

### 2. **GET /api/blocks**

Get all blocks in the blockchain.

**Response:**

```json
{
  "chain": [
    {
      "index": 0,
      "prev_hash": "",
      "timestamp": 1752402232,
      "data": {
        "transaction_table": []
      },
      "nonce": 0,
      "hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0"
    }
    // ... more blocks
  ]
}
```

**Test:**

```bash
curl http://127.0.0.1:3000/api/blocks | jq '.chain | length'
```

---

### 3. **GET /api/blocks/{index}**

Get a specific block by index.

**Response:**

```json
{
  "index": 1,
  "prev_hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0",
  "timestamp": 1752402232,
  "data": {
    "transaction_table": [
      {
        "from": "xarvihs",
        "to": "jarvihs",
        "amount": 1000,
        "fee": 10,
        "signature": null
      }
    ]
  },
  "nonce": 123,
  "hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0"
}
```

**Test:**

```bash
curl http://127.0.0.1:3000/api/blocks/1 | jq '.'
```

---

### 4. **GET /api/transactions**

Get all transactions across all blocks.

**Response:**

```json
[
  {
    "block_index": 1,
    "from": "xarvihs",
    "to": "jarvihs",
    "amount": 1000,
    "fee": 10,
    "block_hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0"
  },
  {
    "block_index": 1,
    "from": "jarvihs",
    "to": "xarvihs",
    "amount": 2000,
    "fee": 20,
    "block_hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0"
  }
]
```

**Test:**

```bash
curl http://127.0.0.1:3000/api/transactions | jq 'length'
```

---

### 5. **GET /api/blocks/{index}/transactions** ⭐ NEW!

Get transactions for a specific block (requires server restart).

**Response:**

```json
[
  {
    "block_index": 1,
    "from": "xarvihs",
    "to": "jarvihs",
    "amount": 1000,
    "fee": 10,
    "block_hash": "00767d5899c8b7118feefe935011f9aa14fbedea0adaafbffe8362eb85a5cde0",
    "signature": null
  }
]
```

**Test:**

```bash
curl http://127.0.0.1:3000/api/blocks/1/transactions | jq '.'
```

---

## 🎯 How to Get Transactions for a Specific Block

### **Current Method (Working):**

```bash
# Get block and extract transactions
curl http://127.0.0.1:3000/api/blocks/1 | jq '.data.transaction_table'
```

### **New Method (After Server Restart):**

```bash
# Direct endpoint for block transactions
curl http://127.0.0.1:3000/api/blocks/1/transactions | jq '.'
```

---

## 🧪 Testing Scripts

### **Quick API Test:**

```bash
#!/bin/bash
echo "🔗 Testing Blockchain API Endpoints"
echo "==================================="

BASE_URL="http://127.0.0.1:3000"

echo ""
echo "1. Status:"
curl -s "$BASE_URL/api/status" | jq '.'

echo ""
echo "2. Total Blocks:"
curl -s "$BASE_URL/api/blocks" | jq '.chain | length'

echo ""
echo "3. Block 1:"
curl -s "$BASE_URL/api/blocks/1" | jq '.index, .hash'

echo ""
echo "4. Block 1 Transactions:"
curl -s "$BASE_URL/api/blocks/1" | jq '.data.transaction_table | length'

echo ""
echo "5. All Transactions:"
curl -s "$BASE_URL/api/transactions" | jq 'length'

echo ""
echo "✅ API test complete!"
```

### **Transaction Analysis:**

```bash
#!/bin/bash
echo "💰 Transaction Analysis"
echo "======================"

BASE_URL="http://127.0.0.1:3000"

echo ""
echo "Total transactions:"
curl -s "$BASE_URL/api/transactions" | jq 'length'

echo ""
echo "Transactions by block:"
for i in {1..9}; do
    count=$(curl -s "$BASE_URL/api/blocks/$i" | jq '.data.transaction_table | length')
    echo "Block $i: $count transactions"
done

echo ""
echo "Largest transaction:"
curl -s "$BASE_URL/api/transactions" | jq 'max_by(.amount) | {from, to, amount}'
```

---

## 🚀 WebSocket Events

### **Real-time Events (ws://127.0.0.1:8080):**

- `BlockMiningStarted`: When mining begins
- `TransactionCreated`: When transactions are created
- `BlockMined`: When blocks are successfully mined
- `BlockchainUpdated`: When blockchain is updated

---

## 📊 Example Usage

### **Get Transaction Statistics:**

```bash
# Total transaction volume
curl -s http://127.0.0.1:3000/api/transactions | \
  jq 'map(.amount) | add'

# Average transaction amount
curl -s http://127.0.0.1:3000/api/transactions | \
  jq 'map(.amount) | add / length'

# Transactions by sender
curl -s http://127.0.0.1:3000/api/transactions | \
  jq 'group_by(.from) | map({from: .[0].from, count: length})'
```

### **Block Analysis:**

```bash
# Get block with most transactions
curl -s http://127.0.0.1:3000/api/blocks | \
  jq '.chain | map({index: .index, tx_count: (.data.transaction_table | length)}) | max_by(.tx_count)'

# Get latest block hash
curl -s http://127.0.0.1:3000/api/blocks | \
  jq '.chain[-1].hash'
```

---

## 🔧 Error Handling

- **404 Not Found**: Block index doesn't exist
- **500 Internal Server Error**: Server error
- **Connection Refused**: Server not running

---

## 🎯 Next Steps

1. **Restart server** to enable new `/api/blocks/{index}/transactions` endpoint
2. **Test WebSocket** connection for real-time updates
3. **Build frontend** to consume these APIs
4. **Add authentication** for secure access
5. **Add pagination** for large datasets

---

**Happy API Testing! 🚀**
