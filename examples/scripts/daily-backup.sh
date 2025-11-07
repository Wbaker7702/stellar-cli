#!/bin/bash
# Daily Backup Script
# Automated backup with notifications

set -e

BACKUP_ROOT="${BACKUP_ROOT:-$HOME/stellar-backups}"
DATE=$(date +%Y%m%d)
BACKUP_DIR="$BACKUP_ROOT/$DATE"

echo "📦 Stellar CLI Daily Backup"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📅 Date: $DATE"
echo "📁 Destination: $BACKUP_DIR"
echo ""

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Get fortune
echo "🔮 Today's fortune..."
stellar fortune > "$BACKUP_DIR/fortune.txt"
cat "$BACKUP_DIR/fortune.txt"
echo ""

# Backup configuration
echo "📤 Exporting Stellar data..."
stellar sync export -o "$BACKUP_DIR/stellar-data"

# Collect stats
echo "📊 Collecting project stats..."
stellar stats > "$BACKUP_DIR/stats.txt" 2>&1 || echo "Stats unavailable" > "$BACKUP_DIR/stats.txt"

# Generate meme
echo "🎭 Generating backup meme..."
stellar meme -t hodl > "$BACKUP_DIR/meme.txt"

# Create backup manifest
cat > "$BACKUP_DIR/BACKUP_INFO.txt" << EOF
Stellar CLI Backup
==================

Date: $(date)
User: $(whoami)
Host: $(hostname)

Contents:
- stellar-data/     Exported Stellar configuration
- stats.txt         Project statistics
- fortune.txt       Today's fortune
- meme.txt          Backup meme
- BACKUP_INFO.txt   This file

Restore:
  stellar sync import -i $BACKUP_DIR/stellar-data

EOF

# Calculate backup size
BACKUP_SIZE=$(du -sh "$BACKUP_DIR" | cut -f1)

echo ""
echo "✅ Backup complete!"
echo "📏 Size: $BACKUP_SIZE"
echo ""

# Send notification
stellar notify send \
    -m "Daily backup complete! Size: $BACKUP_SIZE" \
    -t "Backup Success" \
    -p normal

# Cleanup old backups (keep last 7 days)
echo "🧹 Cleaning old backups..."
find "$BACKUP_ROOT" -maxdepth 1 -type d -mtime +7 -exec rm -rf {} \; 2>/dev/null || true

echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║         💾 BACKUP SUCCESSFUL! 💾                     ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
echo "📁 Location: $BACKUP_DIR"
echo "📱 Check your phone for notification!"
echo ""
