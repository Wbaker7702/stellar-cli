#!/bin/bash
# Stellar CLI - Quick Setup Script
# This script helps you configure notifications for your workflow

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║     🚀 Stellar CLI Notification Setup Wizard 🚀              ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check if stellar CLI is available
if ! command -v stellar &> /dev/null; then
    echo "❌ Stellar CLI not found!"
    echo "   Please build it first: cargo build --release"
    echo "   Then add to PATH or use: ./target/release/stellar"
    exit 1
fi

echo "✅ Stellar CLI found!"
echo ""

# Notification service selection
echo "📲 Choose your notification service:"
echo ""
echo "1. ntfy.sh (Free, self-hosted, easiest setup)"
echo "2. Pushover (Paid, iOS/Android)"
echo "3. Pushbullet (Freemium, cross-platform)"
echo "4. Telegram Bot (Free)"
echo "5. Discord Webhook (Free)"
echo "6. Custom Webhook"
echo ""
read -p "Enter choice [1-6]: " choice

case $choice in
    1)
        echo ""
        echo "🔔 Setting up ntfy.sh..."
        echo ""
        echo "ntfy.sh is a free notification service that works great!"
        echo ""
        
        # Generate unique topic
        TOPIC="stellar-$(whoami)-$(date +%s)"
        WEBHOOK="https://ntfy.sh/$TOPIC"
        
        echo "📱 Your unique topic: $TOPIC"
        echo "🌐 Webhook URL: $WEBHOOK"
        echo ""
        
        # Configure
        stellar notify config --webhook "$WEBHOOK"
        
        echo ""
        echo "✅ Configuration saved!"
        echo ""
        echo "📱 PHONE SETUP:"
        echo "   1. Install ntfy app (iOS/Android)"
        echo "   2. Subscribe to topic: $TOPIC"
        echo "   3. Done! You'll receive notifications"
        echo ""
        echo "   Or use web: https://ntfy.sh/$TOPIC"
        ;;
        
    2)
        echo ""
        echo "🔔 Setting up Pushover..."
        echo ""
        echo "You need a Pushover account and user key."
        echo "Get one at: https://pushover.net/"
        echo ""
        read -p "Enter your Pushover User Key: " userkey
        
        stellar notify config --token "$userkey"
        
        echo ""
        echo "✅ Configuration saved!"
        echo "   Use custom webhook for more options"
        ;;
        
    3)
        echo ""
        echo "🔔 Setting up Pushbullet..."
        echo ""
        echo "You need a Pushbullet access token."
        echo "Get one at: https://www.pushbullet.com/#settings/account"
        echo ""
        read -p "Enter your Pushbullet Access Token: " token
        
        stellar notify config --token "$token"
        
        echo ""
        echo "✅ Configuration saved!"
        ;;
        
    4)
        echo ""
        echo "🔔 Setting up Telegram Bot..."
        echo ""
        echo "Steps:"
        echo "1. Message @BotFather on Telegram"
        echo "2. Create a new bot with /newbot"
        echo "3. Get your bot token"
        echo "4. Get your chat ID (message bot, then check)"
        echo ""
        read -p "Enter Bot Token: " bottoken
        read -p "Enter Chat ID: " chatid
        
        WEBHOOK="https://api.telegram.org/bot$bottoken/sendMessage?chat_id=$chatid"
        stellar notify config --webhook "$WEBHOOK"
        
        echo ""
        echo "✅ Configuration saved!"
        ;;
        
    5)
        echo ""
        echo "🔔 Setting up Discord Webhook..."
        echo ""
        echo "Get webhook URL from Discord:"
        echo "Server Settings → Integrations → Webhooks → New Webhook"
        echo ""
        read -p "Enter Discord Webhook URL: " webhook
        
        stellar notify config --webhook "$webhook"
        
        echo ""
        echo "✅ Configuration saved!"
        ;;
        
    6)
        echo ""
        echo "🔔 Setting up Custom Webhook..."
        echo ""
        read -p "Enter Webhook URL: " webhook
        
        stellar notify config --webhook "$webhook"
        
        echo ""
        echo "✅ Configuration saved!"
        ;;
        
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "🧪 Testing notification setup..."
echo ""

stellar notify test

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                  ✅ Setup Complete! ✅                        ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "📲 You can now send notifications:"
echo ""
echo "   stellar notify send -m \"Hello from Stellar CLI!\""
echo "   stellar notify send -m \"Deployed!\" -p high"
echo ""
echo "💡 Pro Tip: Use in your deployment scripts!"
echo ""
echo "   stellar contract deploy ... && \\"
echo "       stellar notify send -m \"Success!\" -p high"
echo ""
