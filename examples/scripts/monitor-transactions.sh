#!/bin/bash
# Transaction Monitor Script
# Monitor transactions and send notifications

set -e

ACCOUNT="${1}"
CHECK_INTERVAL="${2:-60}"

if [ -z "$ACCOUNT" ]; then
    echo "Usage: $0 <account-address> [check-interval-seconds]"
    exit 1
fi

echo "👁️  Stellar Transaction Monitor"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Account: $ACCOUNT"
echo "⏱️  Interval: ${CHECK_INTERVAL}s"
echo ""

# Send start notification
stellar notify send \
    -m "Started monitoring account: ${ACCOUNT:0:10}..." \
    -t "Monitor Started" \
    -p normal

# Keep track of last seen transaction
LAST_TX=""

while true; do
    echo "🔍 Checking for new transactions... ($(date +%H:%M:%S))"
    
    # Get latest transaction (this is pseudo-code - adjust for actual API)
    # In real implementation, use: stellar events --account $ACCOUNT --latest
    LATEST_TX=$(echo "TX_$(date +%s)" | head -1)
    
    if [ -n "$LATEST_TX" ] && [ "$LATEST_TX" != "$LAST_TX" ]; then
        echo "🆕 New transaction detected!"
        
        # Send notification
        stellar notify send \
            -m "New transaction on account ${ACCOUNT:0:10}..." \
            -t "Transaction Alert" \
            -p high \
            --tx-hash "$LATEST_TX"
        
        # Update last seen
        LAST_TX="$LATEST_TX"
        
        # Optional: Generate QR for viewing on phone
        stellar phone view -a "$ACCOUNT" > /tmp/account-qr.txt
        
        echo "📱 Notification sent!"
    fi
    
    sleep "$CHECK_INTERVAL"
done
