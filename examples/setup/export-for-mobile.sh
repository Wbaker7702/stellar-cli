#!/bin/bash
# Stellar CLI - Export Data Setup Script
# Safely export your Stellar data for mobile access

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║        📦 Stellar CLI Data Export Wizard 📦                  ║"
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

# Export location
DEFAULT_PATH="$HOME/stellar-export"
echo "📁 Export Location"
read -p "   Enter path [$DEFAULT_PATH]: " EXPORT_PATH
EXPORT_PATH=${EXPORT_PATH:-$DEFAULT_PATH}

echo ""
echo "🔐 Export Type"
echo ""
echo "1. Public data only (SAFE - Recommended)"
echo "   • Network configurations"
echo "   • Public keys"
echo "   • Account addresses"
echo "   • Contract info"
echo ""
echo "2. Include private keys (⚠️  CAUTION)"
echo "   • Everything from option 1"
echo "   • Private keys (encrypted)"
echo "   • Full account restore capability"
echo ""
read -p "Choose [1-2]: " export_type

echo ""

if [ "$export_type" = "2" ]; then
    echo "⚠️  WARNING: Private Key Export ⚠️"
    echo ""
    echo "This will export your private keys!"
    echo ""
    echo "You MUST:"
    echo "  1. Keep the export secure"
    echo "  2. Transfer via secure channel only"
    echo "  3. Delete after successful import"
    echo "  4. Never commit to git"
    echo "  5. Use encryption for transfer"
    echo ""
    read -p "Do you understand the risks? [yes/no]: " confirm
    
    if [ "$confirm" != "yes" ]; then
        echo "❌ Export cancelled"
        exit 1
    fi
    
    echo ""
    echo "📤 Exporting with private keys..."
    stellar sync export -o "$EXPORT_PATH" --include-private
else
    echo "📤 Exporting public data only (safe)..."
    stellar sync export -o "$EXPORT_PATH"
fi

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                  ✅ Export Complete! ✅                       ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "📁 Export location: $EXPORT_PATH"
echo ""
echo "📋 What's included:"
ls -lh "$EXPORT_PATH/"
echo ""

if [ "$export_type" = "2" ]; then
    echo "⚠️  SECURITY REMINDER:"
    echo "   • This export contains private keys"
    echo "   • Keep it secure!"
    echo "   • Delete after successful sync"
    echo ""
fi

echo "📱 Transfer to mobile:"
echo ""
echo "   Option 1: QR Code (for viewing only)"
echo "   $ stellar sync qr-code -a <your-account>"
echo ""
echo "   Option 2: File Transfer"
echo "   • Copy $EXPORT_PATH to your phone"
echo "   • Use Stellar mobile app to import"
echo ""
echo "   Option 3: Cloud Sync (if public data only)"
echo "   • Upload to your secure cloud"
echo "   • Download on mobile"
echo ""

# Create a README in export
cat > "$EXPORT_PATH/README.txt" << EOF
STELLAR CLI EXPORT
==================

Created: $(date)
Location: $EXPORT_PATH
Type: $([ "$export_type" = "2" ] && echo "Private keys included" || echo "Public data only")

CONTENTS:
$(ls -1 "$EXPORT_PATH/")

TO IMPORT:
1. Transfer this folder to destination device
2. Run: stellar sync import -i $EXPORT_PATH

$([ "$export_type" = "2" ] && echo "⚠️  SECURITY: This export contains private keys - keep secure!")

EOF

echo "📄 README created: $EXPORT_PATH/README.txt"
echo ""
