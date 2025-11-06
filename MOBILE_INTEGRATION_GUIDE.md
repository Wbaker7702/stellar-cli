# 📱 Mobile & Local Machine Integration Guide

Complete guide for integrating Stellar CLI with your local machine and mobile phone!

## 🎯 Overview

Three powerful new commands for seamless cross-device workflows:

1. **`stellar sync`** - Sync data between devices
2. **`stellar notify`** - Send notifications to your phone  
3. **`stellar phone`** - Full mobile device integration

## 📦 Features Added

### 1. 🔄 Sync Command (`stellar sync`)

Synchronize your Stellar data across devices with secure export/import.

#### Subcommands

**Export Data**
```bash
# Export public data (safe)
stellar sync export

# Export with private keys (USE WITH CAUTION!)
stellar sync export --include-private

# Custom output location
stellar sync export -o /path/to/export
```

**Import Data**
```bash
# Import from directory
stellar sync import -i ./stellar-export

# Import from specific file
stellar sync import -i ./backup/stellar-data.zip
```

**Generate QR Code**
```bash
# Generate QR for account
stellar sync qr-code --account GXXXXXX...

# Specify network
stellar sync qr-code -n mainnet -a GXXXXXX...
```

**Check Status**
```bash
stellar sync status
```

#### What Gets Synced?

✅ Network configurations  
✅ Public keys  
✅ Account information  
✅ Contract addresses  
✅ Custom settings  
⚠️ Private keys (only with `--include-private` flag)

---

### 2. 📲 Notify Command (`stellar notify`)

Send real-time notifications to your phone for important events.

#### Subcommands

**Send Notification**
```bash
# Simple notification
stellar notify send -m "Deployment complete!"

# With custom title
stellar notify send -m "Contract deployed" -t "Success"

# High priority with transaction
stellar notify send -m "Transaction confirmed" -p high --tx-hash ABC123...

# Priority levels: low, normal, high, urgent
```

**Configure Notifications**
```bash
# Set webhook URL
stellar notify config --webhook https://your-webhook.com/notify

# Set phone token
stellar notify config --token your-device-token

# Show current config
stellar notify config --show
```

**Test Setup**
```bash
stellar notify test
```

#### Notification Integrations

Works with popular services:

- **Pushover** - iOS/Android push notifications
- **Pushbullet** - Cross-platform notifications
- **ntfy.sh** - Free, self-hosted notifications
- **Telegram Bot** - Via Telegram API
- **Discord Webhooks** - Discord channel notifications
- **Custom Webhooks** - Any HTTP endpoint

---

### 3. 📱 Phone Command (`stellar phone`)

Complete mobile device integration for signing, viewing, and managing accounts.

#### Subcommands

**Pair Device**
```bash
# Pair with default name
stellar phone pair

# Pair with custom name
stellar phone pair --name "My iPhone"
```

**View Account on Mobile**
```bash
stellar phone view --account GXXXXXX...
```

**Sign Transaction on Phone**
```bash
stellar phone sign --tx <transaction-hash>
```

**Check Mobile Status**
```bash
stellar phone status
```

#### Mobile App Support

Compatible with:
- ✅ Stellar Mobile Wallet
- ✅ Lobstr
- ✅ Freighter Mobile
- ✅ StellarX Mobile

---

## 🚀 Common Use Cases

### Use Case 1: Sync Work & Personal Devices

```bash
# On work computer - export data
stellar sync export -o ~/stellar-backup

# Transfer folder to personal device
# On personal device - import data
stellar sync import -i ~/stellar-backup
```

### Use Case 2: Mobile Monitoring

```bash
# Setup notifications
stellar notify config --webhook https://ntfy.sh/your-topic

# Deploy contract with notification
stellar contract deploy --wasm contract.wasm && \
  stellar notify send -m "Contract deployed!" -p high

# Or use in scripts
if stellar contract invoke ...; then
  stellar notify send -m "Success!" -p normal
else
  stellar notify send -m "Failed!" -p urgent
fi
```

### Use Case 3: Secure Mobile Signing

```bash
# Pair your phone
stellar phone pair --name "iPhone 15"

# Sign transactions on phone (more secure)
stellar contract invoke --id <contract> --sign-on-phone

# Or explicitly:
stellar phone sign --tx <transaction-hash>
```

### Use Case 4: Quick Mobile Access

```bash
# Generate QR for quick mobile access
stellar phone view -a GXXXXXX...

# Scan with mobile app to:
# - View balance
# - See transactions  
# - Quick send/receive
```

### Use Case 5: CI/CD Notifications

```bash
#!/bin/bash
# In your CI/CD pipeline

# Run tests
if cargo test; then
  stellar notify send -m "Tests passed ✓" -p normal
else
  stellar notify send -m "Tests failed ✗" -p urgent
fi

# Deploy
if stellar contract deploy ...; then
  stellar notify send -m "Deployed to mainnet 🚀" -p high
fi
```

---

## 🔐 Security Best Practices

### Private Key Handling

⚠️ **NEVER** include private keys in exports unless absolutely necessary!

**Safe Export (Recommended)**
```bash
# Public data only - safe to transfer
stellar sync export
```

**Private Export (Use with extreme caution)**
```bash
# Only when you need to restore full access
stellar sync export --include-private

# MUST:
# 1. Transfer via secure channel
# 2. Delete export after import
# 3. Verify import success
# 4. Use strong encryption
```

### Mobile Security

✅ Enable biometric authentication  
✅ Use device pairing with verification  
✅ Review transactions before signing  
✅ Keep apps updated  
✅ Use official app stores only

### Notification Security

- Use HTTPS webhooks only
- Keep tokens secure (never commit to git)
- Use environment variables for sensitive data
- Monitor notification delivery logs

---

## 🎨 Integration Examples

### Example 1: Automated Backup Script

```bash
#!/bin/bash
# backup-stellar.sh

DATE=$(date +%Y%m%d)
BACKUP_DIR="$HOME/stellar-backups/$DATE"

echo "📦 Creating Stellar backup..."
stellar sync export -o "$BACKUP_DIR"

echo "📲 Sending notification..."
stellar notify send -m "Backup created: $DATE" -t "Stellar Backup"

echo "✅ Backup complete: $BACKUP_DIR"
```

### Example 2: Deployment with Notification

```bash
#!/bin/bash
# deploy-with-notify.sh

CONTRACT_PATH="$1"

echo "🚀 Deploying contract..."
if stellar contract deploy --wasm "$CONTRACT_PATH"; then
    stellar notify send \
        -m "Contract deployed successfully!" \
        -t "Deployment Success" \
        -p high
    exit 0
else
    stellar notify send \
        -m "Contract deployment failed!" \
        -t "Deployment Failed" \
        -p urgent
    exit 1
fi
```

### Example 3: Mobile Transaction Monitor

```bash
#!/bin/bash
# monitor-transactions.sh

ACCOUNT="$1"

while true; do
    # Check for new transactions
    stellar events --account "$ACCOUNT" --latest | \
    while read -r event; do
        stellar notify send \
            -m "New transaction detected" \
            -t "Account Activity" \
            -p normal
    done
    
    sleep 60
done
```

### Example 4: QR Code Generator for Events

```bash
#!/bin/bash
# generate-event-qr.sh

echo "📱 Generating QR codes for attendees..."

for account in $(cat attendee-accounts.txt); do
    echo "Generating QR for $account"
    stellar sync qr-code -a "$account" > "qr-$account.txt"
done

echo "✅ QR codes generated!"
```

---

## 🛠️ Setup Instructions

### 1. Setup Notifications (Pushover Example)

```bash
# Sign up at pushover.net and get your user key

# Configure
stellar notify config --token your-user-key

# Test
stellar notify test

# If using webhook mode
stellar notify config --webhook https://api.pushover.net/1/messages.json
```

### 2. Setup Notifications (ntfy.sh - Free!)

```bash
# Choose a unique topic
TOPIC="stellar-$(whoami)-$(date +%s)"

# Configure
stellar notify config --webhook https://ntfy.sh/$TOPIC

# Install ntfy app on phone
# Subscribe to your topic: $TOPIC

# Test
stellar notify test
```

### 3. Pair Mobile Device

```bash
# On computer
stellar phone pair --name "My Phone"

# On phone
# 1. Open Stellar app
# 2. Settings → Pair Device
# 3. Enter pairing code
# 4. Confirm

# Verify
stellar phone status
```

---

## 📊 Command Reference

### Sync Commands

| Command | Description |
|---------|-------------|
| `stellar sync export` | Export data for mobile |
| `stellar sync import -i <path>` | Import data from mobile |
| `stellar sync qr-code -a <account>` | Generate QR code |
| `stellar sync status` | Show sync status |

### Notify Commands

| Command | Description |
|---------|-------------|
| `stellar notify send -m <msg>` | Send notification |
| `stellar notify config --show` | Show configuration |
| `stellar notify config --webhook <url>` | Set webhook |
| `stellar notify test` | Test notifications |

### Phone Commands

| Command | Description |
|---------|-------------|
| `stellar phone pair` | Pair mobile device |
| `stellar phone view -a <account>` | View on mobile |
| `stellar phone sign --tx <hash>` | Sign on phone |
| `stellar phone status` | Check mobile status |

---

## 💡 Pro Tips

1. **Automated Backups**: Set up cron job for daily syncs
   ```bash
   0 2 * * * /usr/local/bin/stellar sync export -o ~/backups/stellar
   ```

2. **Notification Groups**: Use different webhooks for different priorities
   ```bash
   # Low priority - Discord
   stellar notify config --webhook https://discord.com/api/webhooks/...
   
   # High priority - Pushover
   stellar notify config --webhook https://api.pushover.net/...
   ```

3. **QR Code Sharing**: Generate QR codes for easy account sharing at events

4. **Mobile-First Signing**: Keep private keys on phone, sign from CLI

5. **Status Dashboard**: Combine with scripts to create status monitors

---

## 🔍 Troubleshooting

### Notifications Not Working?

```bash
# Check configuration
stellar notify config --show

# Test connection
stellar notify test

# Verify webhook/token
# - Check URL is accessible
# - Verify token is valid
# - Check phone notifications are enabled
```

### Sync Issues?

```bash
# Check status
stellar sync status

# Verify export directory exists
ls -la ./stellar-export

# Check import path
file stellar-export/MANIFEST.txt
```

### Phone Pairing Failed?

```bash
# Check status
stellar phone status

# Try new pairing
stellar phone pair --name "New Pairing"

# Verify mobile app is updated
```

---

## 🎉 Summary

You now have complete integration between:
- 💻 **Local Machine** - Full CLI control
- 📱 **Mobile Phone** - Secure signing & monitoring
- 🔄 **Cross-Device Sync** - Seamless data transfer
- 📲 **Real-time Notifications** - Stay informed anywhere

**Next Steps:**
1. Set up notifications for your workflow
2. Pair your mobile device
3. Create automated sync scripts
4. Enjoy seamless cross-device experience!

---

*Made with ❤️ for the Stellar community*

*🚀 To the moon, from any device! 🌕*
