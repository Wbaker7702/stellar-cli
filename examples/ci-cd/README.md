# Stellar CLI - CI/CD Examples

This directory contains example CI/CD configurations for integrating Stellar CLI into your automated workflows.

## 📋 Available Examples

### GitHub Actions (`github-actions.yml`)

Complete GitHub Actions workflow with:
- ✅ Build and test automation
- 🚀 Automated deployments
- 📲 Notification integration
- 📦 Scheduled backups
- 🔮 Fortune and meme generation

**Setup:**
1. Copy to `.github/workflows/stellar.yml`
2. Add secrets:
   - `STELLAR_SECRET_KEY` - Your deployment key
   - `NOTIFICATION_WEBHOOK` - Your notification webhook URL
3. Push to trigger workflow

### GitLab CI (`gitlab-ci.yml`)

Complete GitLab CI/CD pipeline with:
- 🔨 Multi-stage build process
- 🧪 Automated testing
- 🚀 Testnet and mainnet deployment
- 📲 Notification hooks
- 💾 Artifact management

**Setup:**
1. Copy to `.gitlab-ci.yml` in your project root
2. Configure CI/CD variables:
   - `STELLAR_SECRET_KEY`
   - `NOTIFICATION_WEBHOOK`
3. Commit and push

## 🔐 Security Best Practices

### Secrets Management

**Never commit secrets to your repository!**

Store secrets securely:

**GitHub Actions:**
```bash
# Settings → Secrets and variables → Actions → New repository secret
STELLAR_SECRET_KEY=SXXXXXX...
NOTIFICATION_WEBHOOK=https://...
```

**GitLab CI:**
```bash
# Settings → CI/CD → Variables → Add variable
STELLAR_SECRET_KEY (Protected, Masked)
NOTIFICATION_WEBHOOK (Protected)
```

### Environment-Specific Deployments

Use different keys and networks:
- **Development** → Local/Futurenet
- **Staging** → Testnet
- **Production** → Mainnet (manual approval required)

## 🚀 Quick Start

### 1. Install Stellar CLI in CI

Add to your workflow:

```yaml
- name: Install Stellar CLI
  run: |
    curl -L https://github.com/stellar/stellar-cli/releases/latest/download/stellar-cli-linux.tar.gz | tar xz
    sudo mv stellar /usr/local/bin/
```

Or use cargo:

```yaml
- name: Install Stellar CLI
  run: cargo install stellar-cli
```

### 2. Configure Notifications

```yaml
- name: Setup notifications
  run: |
    stellar notify config --webhook ${{ secrets.NOTIFICATION_WEBHOOK }}
```

### 3. Deploy with Notifications

```yaml
- name: Deploy contract
  run: |
    stellar contract deploy --wasm contract.wasm && \
      stellar notify send -m "Deployed!" -p high || \
      stellar notify send -m "Failed!" -p urgent
```

## 📱 Notification Integration Examples

### ntfy.sh (Free)

```yaml
env:
  NOTIFICATION_WEBHOOK: https://ntfy.sh/your-topic
```

### Discord

```yaml
env:
  NOTIFICATION_WEBHOOK: https://discord.com/api/webhooks/...
```

### Telegram

```yaml
env:
  NOTIFICATION_WEBHOOK: https://api.telegram.org/bot$TOKEN/sendMessage?chat_id=$CHAT_ID
```

### Pushover

```yaml
- name: Send notification
  run: |
    stellar notify config --token ${{ secrets.PUSHOVER_TOKEN }}
    stellar notify send -m "Message" -p high
```

## 🎯 Common Workflows

### On Push to Main → Deploy to Testnet

```yaml
on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Deploy
        run: |
          stellar contract deploy --network testnet
          stellar notify send -m "Deployed to testnet" -p high
          stellar meme -t moon
```

### On Tag → Deploy to Mainnet

```yaml
on:
  push:
    tags:
      - 'v*'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Get fortune
        run: stellar fortune -t blockchain
      - name: Deploy to mainnet
        run: |
          stellar contract deploy --network mainnet
          stellar notify send -m "🚀 MAINNET DEPLOYED!" -p urgent
          stellar meme -t chad
```

### Scheduled Daily Backup

```yaml
on:
  schedule:
    - cron: '0 2 * * *'  # 2 AM UTC daily

jobs:
  backup:
    runs-on: ubuntu-latest
    steps:
      - name: Backup
        run: |
          stellar sync export
          stellar notify send -m "Backup complete" -p normal
```

### On PR → Run Tests with Notifications

```yaml
on:
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Test
        run: cargo test
      - name: Notify
        if: success()
        run: |
          stellar notify send -m "PR tests passed ✅" -p normal
          stellar meme -t hodl
      - name: Notify failure
        if: failure()
        run: |
          stellar notify send -m "PR tests failed ❌" -p high
          stellar meme -t wojak
```

## 💡 Pro Tips

1. **Use caching** to speed up builds:
   ```yaml
   - uses: actions/cache@v3
     with:
       path: |
         ~/.cargo
         target
       key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
   ```

2. **Add fortune checks** before important operations:
   ```yaml
   - name: Cosmic guidance
     run: stellar fortune -t blockchain
   ```

3. **Generate QR codes** for deployed contracts:
   ```yaml
   - name: Mobile QR
     run: stellar sync qr-code -a $CONTRACT_ID
   ```

4. **Use different notification priorities**:
   - `low` - Informational
   - `normal` - Standard updates
   - `high` - Important events
   - `urgent` - Critical alerts

5. **Add memes for team morale**:
   ```yaml
   - name: Celebrate
     if: success()
     run: stellar meme -t moon
   ```

## 🔍 Debugging CI/CD

If things go wrong:

1. **Check secrets are set**:
   ```yaml
   - name: Verify secrets
     run: |
       echo "Webhook configured: ${{ secrets.NOTIFICATION_WEBHOOK != '' }}"
   ```

2. **Test notification setup**:
   ```yaml
   - name: Test notifications
     run: stellar notify test
   ```

3. **Add verbose logging**:
   ```yaml
   - name: Deploy with logging
     run: stellar contract deploy --verbose
   ```

4. **Save artifacts for inspection**:
   ```yaml
   - uses: actions/upload-artifact@v3
     with:
       name: deployment-logs
       path: logs/
   ```

## 📚 Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [GitLab CI/CD Documentation](https://docs.gitlab.com/ee/ci/)
- [Stellar CLI Documentation](../../FULL_HELP_DOCS.md)
- [Mobile Integration Guide](../../MOBILE_INTEGRATION_GUIDE.md)

---

**Happy Automating! 🚀**

*Remember: Always test in development before deploying to mainnet!*
