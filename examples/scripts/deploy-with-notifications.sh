#!/bin/bash
# Automated Deployment Script with Notifications
# Deploy smart contracts with full notification integration

set -e

CONTRACT_WASM="${1:-contract.wasm}"
NETWORK="${2:-testnet}"

echo "🚀 Stellar Smart Contract Deployment"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Contract: $CONTRACT_WASM"
echo "🌐 Network:  $NETWORK"
echo ""

# Check if contract exists
if [ ! -f "$CONTRACT_WASM" ]; then
    echo "❌ Contract file not found: $CONTRACT_WASM"
    stellar notify send -m "Deployment failed: Contract not found" -p urgent 2>/dev/null || true
    exit 1
fi

# Get fortune for guidance
echo "🔮 Consulting the blockchain oracle..."
stellar fortune -t blockchain
echo ""

# Pre-deployment notification
stellar notify send \
    -m "Starting deployment of $CONTRACT_WASM to $NETWORK" \
    -t "Deployment Started" \
    -p normal

echo "📊 Pre-deployment stats..."
stellar stats | head -10
echo ""

# Build contract
echo "🔨 Building contract..."
if cargo build --release --target wasm32-unknown-unknown; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    stellar notify send \
        -m "Contract build failed!" \
        -t "Build Error" \
        -p urgent
    stellar meme -t rekt
    exit 1
fi

# Run tests
echo ""
echo "🧪 Running tests..."
if cargo test; then
    echo "✅ Tests passed"
    stellar notify send \
        -m "Tests passed, proceeding with deployment" \
        -p normal
else
    echo "❌ Tests failed"
    stellar notify send \
        -m "Tests failed! Deployment aborted" \
        -t "Test Failure" \
        -p urgent
    stellar meme -t wojak
    exit 1
fi

# Deploy
echo ""
echo "🚀 Deploying to $NETWORK..."
if DEPLOY_OUTPUT=$(stellar contract deploy \
    --wasm "$CONTRACT_WASM" \
    --network "$NETWORK" 2>&1); then
    
    echo "✅ Deployment successful!"
    echo "$DEPLOY_OUTPUT"
    
    # Extract contract ID (example parsing)
    CONTRACT_ID=$(echo "$DEPLOY_OUTPUT" | grep -o 'C[A-Z0-9]\{55\}' | head -1 || echo "CONTRACT_ID_HERE")
    
    # Success notification with contract ID
    stellar notify send \
        -m "🎉 Contract deployed successfully! ID: $CONTRACT_ID" \
        -t "Deployment Success" \
        -p high
    
    # Generate QR for mobile access
    echo ""
    echo "📱 Generating QR code for mobile access..."
    stellar sync qr-code -a "$CONTRACT_ID" 2>/dev/null || true
    
    # Celebrate!
    stellar meme -t moon
    
    # Export deployment info
    echo ""
    echo "💾 Saving deployment info..."
    cat > "deployment-$(date +%Y%m%d-%H%M%S).json" << EOF
{
  "contract": "$CONTRACT_WASM",
  "network": "$NETWORK",
  "contract_id": "$CONTRACT_ID",
  "deployed_at": "$(date -Iseconds)",
  "deployed_by": "$(whoami)"
}
EOF
    
    echo "✅ Deployment complete!"
    
else
    echo "❌ Deployment failed"
    echo "$DEPLOY_OUTPUT"
    
    # Failure notification
    stellar notify send \
        -m "Deployment failed! Check logs" \
        -t "Deployment Failed" \
        -p urgent
    
    stellar meme -t rekt
    exit 1
fi

echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║         🎉 DEPLOYMENT SUCCESSFUL! 🎉                 ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
echo "📝 Contract ID: $CONTRACT_ID"
echo "🌐 Network: $NETWORK"
echo "📱 Check your phone for notifications!"
echo ""
