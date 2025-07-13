#!/bin/bash

echo "🔗 Blockchain Simulator - Complete API Test"
echo "==========================================="

BASE_URL="http://127.0.0.1:3000"

# Check if server is running
echo ""
echo "1. Checking server status..."
if curl -s "$BASE_URL/api/status" > /dev/null; then
    echo "✅ Server is running"
else
    echo "❌ Server is not running. Please start with: cargo run"
    exit 1
fi

echo ""
echo "2. Testing /api/status endpoint..."
status=$(curl -s "$BASE_URL/api/status")
echo "   Total blocks: $(echo $status | jq -r '.total_blocks')"
echo "   Connected clients: $(echo $status | jq -r '.connected_clients')"
echo "   Last block hash: $(echo $status | jq -r '.last_block_hash' | cut -c1-16)..."

echo ""
echo "3. Testing /api/blocks endpoint..."
blocks=$(curl -s "$BASE_URL/api/blocks")
total_blocks=$(echo $blocks | jq '.chain | length')
echo "   Total blocks in chain: $total_blocks"

echo ""
echo "4. Testing /api/blocks/{index} endpoints..."
for i in {1..3}; do
    block=$(curl -s "$BASE_URL/api/blocks/$i")
    if [ $? -eq 0 ]; then
        index=$(echo $block | jq -r '.index')
        hash=$(echo $block | jq -r '.hash' | cut -c1-16)
        tx_count=$(echo $block | jq '.data.transaction_table | length')
        echo "   Block $i: index=$index, hash=${hash}..., transactions=$tx_count"
    else
        echo "   Block $i: ❌ Not found"
    fi
done

echo ""
echo "5. Testing /api/transactions endpoint..."
transactions=$(curl -s "$BASE_URL/api/transactions")
total_tx=$(echo $transactions | jq 'length')
echo "   Total transactions: $total_tx"

if [ $total_tx -gt 0 ]; then
    echo "   Sample transaction:"
    echo $transactions | jq '.[0] | {from, to, amount, fee, block_index}'
fi

echo ""
echo "6. Testing block transaction extraction..."
for i in {1..3}; do
    block_tx=$(curl -s "$BASE_URL/api/blocks/$i" | jq '.data.transaction_table | length')
    echo "   Block $i transactions: $block_tx"
done

echo ""
echo "7. Transaction Analysis..."
if [ $total_tx -gt 0 ]; then
    echo "   Total volume: $(echo $transactions | jq 'map(.amount) | add')"
    echo "   Average amount: $(echo $transactions | jq 'map(.amount) | add / length')"
    echo "   Largest transaction: $(echo $transactions | jq 'max_by(.amount) | .amount')"
fi

echo ""
echo "8. Testing /api/blocks/{index}/transactions endpoint..."
echo "   Note: This endpoint requires server restart to work"
for i in {1..2}; do
    response=$(curl -s -w "%{http_code}" "$BASE_URL/api/blocks/$i/transactions")
    http_code="${response: -3}"
    if [ "$http_code" = "200" ]; then
        tx_count=$(echo ${response%???} | jq 'length')
        echo "   Block $i transactions: $tx_count (✅ Working)"
    else
        echo "   Block $i transactions: ❌ Not available (HTTP $http_code)"
    fi
done

echo ""
echo "9. WebSocket Status..."
ws_status=$(curl -s "$BASE_URL/api/status" | jq -r '.connected_clients')
echo "   Connected WebSocket clients: $ws_status"

echo ""
echo "✅ API test complete!"
echo ""
echo "📊 Summary:"
echo "   - Total blocks: $total_blocks"
echo "   - Total transactions: $total_tx"
echo "   - WebSocket clients: $ws_status"
echo ""
echo "🌐 Available endpoints:"
echo "   - GET /api/status"
echo "   - GET /api/blocks"
echo "   - GET /api/blocks/{index}"
echo "   - GET /api/transactions"
echo "   - GET /api/blocks/{index}/transactions (after restart)"
echo ""
echo "🔌 WebSocket: ws://127.0.0.1:8080"
echo "📊 Dashboard: Open index.html in your browser" 