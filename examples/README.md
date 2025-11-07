# 🚀 Stellar CLI - Examples & Quick Start Guide

Complete collection of examples, scripts, and setup guides to get you started with all the new Stellar CLI features!

## 📁 Directory Structure

```
examples/
├── setup/              Setup wizard scripts
│   ├── setup-notifications.sh
│   ├── export-for-mobile.sh
│   └── pair-phone.sh
├── scripts/            Automation scripts
│   ├── deploy-with-notifications.sh
│   ├── daily-backup.sh
│   ├── monitor-transactions.sh
│   └── fortune-driven-dev.sh
└── ci-cd/              CI/CD integrations
    ├── github-actions.yml
    ├── gitlab-ci.yml
    └── README.md
```

## 🎯 Quick Start - 5 Steps to Full Integration

### Step 1: Configure Notifications

Run the interactive setup wizard:

```bash
chmod +x examples/setup/setup-notifications.sh
./examples/setup/setup-notifications.sh
```

This will guide you through:
- ✅ Choosing a notification service (ntfy.sh, Pushover, etc.)
- ✅ Configuring webhooks/tokens
- ✅ Testing notification delivery
- ✅ Getting started tips

**Or manually:**
```bash
# Free option - ntfy.sh
stellar notify config --webhook https://ntfy.sh/your-unique-topic

# Test it
stellar notify test
```

---

### Step 2: Export Your Data for Mobile

Run the export wizard:

```bash
chmod +x examples/setup/export-for-mobile.sh
./examples/setup/export-for-mobile.sh
```

This will:
- ✅ Export your Stellar configuration
- ✅ Choose between public-only (safe) or with private keys
- ✅ Create a backup manifest
- ✅ Generate transfer instructions

**Or manually:**
```bash
# Safe export (public data only)
stellar sync export

# With private keys (use caution!)
stellar sync export --include-private
```

---

### Step 3: Pair Your Phone

Run the pairing wizard:

```bash
chmod +x examples/setup/pair-phone.sh
./examples/setup/pair-phone.sh
```

This will:
- ✅ Generate pairing code or QR
- ✅ Provide step-by-step phone instructions
- ✅ Verify connection

**Or manually:**
```bash
# Generate pairing code
stellar phone pair --name "My iPhone"

# Or generate QR code
stellar sync qr-code -a <your-account>
```

---

### Step 4: Try the Examples

Test the fun features:

```bash
# Get your fortune
stellar fortune -t moon

# Check project stats
stellar stats --animated

# Generate a meme
stellar meme -t hodl
```

Test integration features:

```bash
# Send notification
stellar notify send -m "Hello from Stellar CLI!" -p normal

# Check phone status
stellar phone status

# View account on mobile
stellar phone view -a <account-address>
```

---

### Step 5: Integrate into Your Workflow

Choose from our example scripts:

**Automated Deployment:**
```bash
chmod +x examples/scripts/deploy-with-notifications.sh
./examples/scripts/deploy-with-notifications.sh contract.wasm testnet
```

**Daily Backups:**
```bash
chmod +x examples/scripts/daily-backup.sh
./examples/scripts/daily-backup.sh

# Or add to cron:
# 0 2 * * * /path/to/daily-backup.sh
```

**Fortune-Driven Development:**
```bash
chmod +x examples/scripts/fortune-driven-dev.sh
./examples/scripts/fortune-driven-dev.sh
```

---

## 📜 Available Scripts

### 🛠️ Setup Scripts

#### `setup-notifications.sh`
Interactive wizard for configuring notification services.

**Supports:**
- ntfy.sh (Free)
- Pushover (iOS/Android)
- Pushbullet (Cross-platform)
- Telegram Bot
- Discord Webhooks
- Custom webhooks

**Usage:**
```bash
./examples/setup/setup-notifications.sh
```

---

#### `export-for-mobile.sh`
Export Stellar data for mobile sync with safety checks.

**Features:**
- Public-only or with private keys
- Safety warnings and confirmations
- Automatic manifest generation
- Transfer instructions

**Usage:**
```bash
./examples/setup/export-for-mobile.sh
```

---

#### `pair-phone.sh`
Pair mobile device with CLI for secure signing.

**Methods:**
- Pairing code (manual entry)
- QR code (scan to pair)
- Account view QR (quick access)

**Usage:**
```bash
./examples/setup/pair-phone.sh
```

---

### 🤖 Automation Scripts

#### `deploy-with-notifications.sh`
Complete deployment pipeline with notifications.

**Features:**
- Pre-deployment fortune check
- Build and test validation
- Deployment with notifications
- Success/failure memes
- QR code generation
- Deployment info logging

**Usage:**
```bash
./examples/scripts/deploy-with-notifications.sh contract.wasm testnet
```

---

#### `daily-backup.sh`
Automated daily backup script.

**Includes:**
- Stellar data export
- Project statistics
- Daily fortune
- Backup meme
- Notification on completion
- Old backup cleanup (7 days)

**Usage:**
```bash
./examples/scripts/daily-backup.sh

# Add to cron for automation:
# 0 2 * * * /path/to/daily-backup.sh
```

---

#### `monitor-transactions.sh`
Real-time transaction monitoring with notifications.

**Features:**
- Continuous monitoring
- New transaction detection
- Push notifications
- QR code generation

**Usage:**
```bash
./examples/scripts/monitor-transactions.sh <account-address> [interval-seconds]

# Example:
./examples/scripts/monitor-transactions.sh GXXXXXX... 60
```

---

#### `fortune-driven-dev.sh`
Let cosmic forces guide your commits! 🔮

**How it works:**
- Gets fortune before commit
- Favorable fortune → Proceed
- Cautious fortune → Run tests first
- Unfavorable fortune → Reconsider

**Usage:**
```bash
# Stage your changes
git add .

# Let the fortune decide
./examples/scripts/fortune-driven-dev.sh
```

---

### 🔄 CI/CD Examples

See `examples/ci-cd/README.md` for complete CI/CD integration guides.

**Available:**
- GitHub Actions workflow
- GitLab CI pipeline
- Notification integration
- Automated deployments
- Scheduled backups

---

## 💡 Usage Patterns

### Pattern 1: Development Workflow

```bash
# Morning routine
stellar fortune -t blockchain
stellar stats --animated

# Make changes...

# Pre-commit check
./examples/scripts/fortune-driven-dev.sh

# Deploy
./examples/scripts/deploy-with-notifications.sh contract.wasm testnet
```

---

### Pattern 2: Production Deployment

```bash
# 1. Get cosmic approval
stellar fortune -t blockchain

# 2. Final tests
cargo test

# 3. Deploy
stellar contract deploy --wasm contract.wasm --network mainnet

# 4. Notify team
stellar notify send -m "🚀 Deployed to mainnet!" -p urgent

# 5. Generate mobile QR
stellar sync qr-code -a <contract-id>

# 6. Celebrate
stellar meme -t moon
```

---

### Pattern 3: Monitoring Setup

```bash
# Terminal 1: Monitor transactions
./examples/scripts/monitor-transactions.sh <account> 60

# Terminal 2: Watch stats
watch -n 300 stellar stats

# Notifications on your phone!
```

---

### Pattern 4: Team Collaboration

```bash
# Export config for team member
stellar sync export -o ./team-share

# Share the folder (public data only is safe)

# Team member imports
stellar sync import -i ./team-share

# Everyone gets notifications
stellar notify config --webhook <team-webhook>
```

---

## 🎨 Customization

### Create Your Own Scripts

All examples are templates. Customize them:

```bash
# Copy an example
cp examples/scripts/deploy-with-notifications.sh my-deploy.sh

# Edit for your needs
vim my-deploy.sh

# Make executable
chmod +x my-deploy.sh

# Use it!
./my-deploy.sh
```

### Common Customizations

**Change notification priorities:**
```bash
# In scripts, modify:
stellar notify send -m "Message" -p high  # or: low, normal, urgent
```

**Add more fortune themes:**
```bash
stellar fortune -t blockchain  # blockchain, stellar, moon, lambo
```

**Customize meme celebrations:**
```bash
stellar meme -t moon  # hodl, doge, moon, rekt, chad, wojak
```

---

## 🔐 Security Notes

### Private Key Handling

**NEVER:**
- ❌ Commit private keys to git
- ❌ Share private exports publicly
- ❌ Store unencrypted in CI/CD

**ALWAYS:**
- ✅ Use `--include-private` flag explicitly
- ✅ Delete private exports after sync
- ✅ Transfer via secure channels
- ✅ Use CI/CD secrets for keys

### Notification Security

**Best Practices:**
- Use HTTPS webhooks only
- Store tokens in environment variables
- Don't include sensitive data in messages
- Use appropriate priority levels

---

## 🐛 Troubleshooting

### Scripts not executable?

```bash
chmod +x examples/**/*.sh
```

### Stellar CLI not found?

```bash
# Build it
cargo build --release

# Add to PATH
export PATH="$PWD/target/release:$PATH"

# Or use full path
./target/release/stellar
```

### Notifications not working?

```bash
# Check config
stellar notify config --show

# Test connection
stellar notify test

# Verify webhook URL
curl -X POST <your-webhook-url> -d '{"message":"test"}'
```

### Sync issues?

```bash
# Check status
stellar sync status

# Verify export
ls -la stellar-export/
cat stellar-export/MANIFEST.txt
```

---

## 📚 Additional Resources

**Documentation:**
- [NEW_FEATURES.md](../NEW_FEATURES.md) - Fun features guide
- [MOBILE_INTEGRATION_GUIDE.md](../MOBILE_INTEGRATION_GUIDE.md) - Mobile integration
- [ALL_NEW_FEATURES.md](../ALL_NEW_FEATURES.md) - Complete feature list
- [QUICK_REFERENCE.md](../QUICK_REFERENCE.md) - Command reference

**Help:**
```bash
stellar fortune --help
stellar stats --help
stellar meme --help
stellar sync --help
stellar notify --help
stellar phone --help
```

---

## 🎉 Community Contributions

Have a cool script or workflow? Share it!

1. Create your script in appropriate directory
2. Add documentation
3. Submit a pull request
4. Help others automate their workflows!

---

## ✨ Quick Tips

1. **Start simple:** Try fun features first (fortune, stats, meme)
2. **Add notifications:** Easy wins for awareness
3. **Automate backups:** Set up daily-backup.sh
4. **Integrate CI/CD:** Use our examples as templates
5. **Experiment:** All scripts are safe to modify
6. **Have fun:** Don't forget the memes! 🚀

---

**Happy Automating! 🌟**

*Made with ❤️ for the Stellar community*

*"From manual commands to automated workflows in 5 steps!"* 🚀
