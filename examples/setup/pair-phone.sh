#!/bin/bash
# Stellar CLI - Phone Pairing Setup Script
# Pair your mobile device for secure signing

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║         📱 Stellar CLI Phone Pairing Wizard 📱               ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check if stellar CLI is available
if ! command -v stellar &> /dev/null; then
    echo "❌ Stellar CLI not found!"
    exit 1
fi

echo "✅ Stellar CLI found!"
echo ""

# Get device name
echo "📱 Device Information"
read -p "   Enter device name (e.g., iPhone, Pixel): " device_name
device_name=${device_name:-"My Phone"}

echo ""
echo "🔗 Pairing Method"
echo ""
echo "1. Pairing Code (Manual entry)"
echo "2. QR Code (Scan with phone)"
echo "3. Account View QR (Quick access)"
echo ""
read -p "Choose [1-3]: " method

echo ""

case $method in
    1)
        echo "📋 Generating pairing code..."
        echo ""
        stellar phone pair --name "$device_name"
        echo ""
        echo "📱 On Your Phone:"
        echo "   1. Open Stellar mobile app"
        echo "   2. Go to Settings → Pair Device"
        echo "   3. Enter the code above"
        echo "   4. Confirm pairing"
        ;;
        
    2)
        echo "📱 Generating QR code..."
        echo ""
        
        # Get account if available
        read -p "Enter account address (or press Enter to skip): " account
        
        if [ -n "$account" ]; then
            stellar sync qr-code -a "$account"
        else
            stellar sync qr-code
        fi
        
        echo ""
        echo "📱 On Your Phone:"
        echo "   1. Open Stellar mobile app"
        echo "   2. Tap 'Scan QR Code'"
        echo "   3. Point camera at QR above"
        echo "   4. Confirm connection"
        ;;
        
    3)
        echo "📱 Quick Account View QR..."
        echo ""
        read -p "Enter account address: " account
        
        if [ -z "$account" ]; then
            echo "❌ Account address required"
            exit 1
        fi
        
        stellar phone view -a "$account"
        
        echo ""
        echo "📱 On Your Phone:"
        echo "   • Scan QR with any QR reader"
        echo "   • Or use Stellar app's scan feature"
        echo "   • Instant account access!"
        ;;
        
    *)
        echo "❌ Invalid choice"
        exit 1
        ;;
esac

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║               🔗 Pairing Information Sent! 🔗                ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "✅ Next Steps:"
echo ""
echo "   1. Complete pairing on your phone"
echo "   2. Verify connection:"
echo "      $ stellar phone status"
echo ""
echo "   3. Try viewing an account:"
echo "      $ stellar phone view -a <address>"
echo ""
echo "   4. Sign transactions securely:"
echo "      $ stellar phone sign --tx <hash>"
echo ""
echo "🔐 Benefits:"
echo "   • Private keys stay on phone"
echo "   • Biometric authentication"
echo "   • Review transactions on mobile"
echo "   • Enhanced security"
echo ""
echo "💡 Supported Apps:"
echo "   • Stellar Mobile Wallet"
echo "   • Lobstr"
echo "   • Freighter Mobile"
echo "   • StellarX Mobile"
echo ""
