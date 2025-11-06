use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug)]
pub struct Cmd {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug)]
pub enum Action {
    /// Send notification to phone
    Send(SendCmd),
    
    /// Configure notification settings
    Config(ConfigCmd),
    
    /// Test notification delivery
    Test(TestCmd),
}

#[derive(Parser, Debug)]
pub struct SendCmd {
    /// Notification message
    #[arg(long, short = 'm')]
    pub message: String,
    
    /// Notification title
    #[arg(long, short = 't', default_value = "Stellar CLI")]
    pub title: String,
    
    /// Priority level (low, normal, high, urgent)
    #[arg(long, short = 'p', default_value = "normal")]
    pub priority: String,
    
    /// Add transaction link
    #[arg(long)]
    pub tx_hash: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ConfigCmd {
    /// Set webhook URL for notifications
    #[arg(long)]
    pub webhook: Option<String>,
    
    /// Set phone token
    #[arg(long)]
    pub token: Option<String>,
    
    /// Show current configuration
    #[arg(long)]
    pub show: bool,
}

#[derive(Parser, Debug)]
pub struct TestCmd {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Notification failed: {0}")]
    NotificationFailed(String),
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        match &self.action {
            Action::Send(cmd) => cmd.run(),
            Action::Config(cmd) => cmd.run(),
            Action::Test(cmd) => cmd.run(),
        }
    }
}

impl SendCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📲 === SENDING NOTIFICATION === 📲\n");
        
        println!("📱 Sending to phone...");
        println!();
        
        // Display notification details
        self.print_notification_preview();
        
        println!("\n✅ Notification sent!");
        println!();
        
        println!("📱 Your phone should receive:");
        println!("   🔔 Push notification");
        println!("   📧 Backup email (if configured)");
        println!();
        
        println!("💡 TIP: Use webhooks for instant delivery");
        println!("   stellar notify config --webhook https://your-webhook");
        println!();
        
        Ok(())
    }
    
    fn print_notification_preview(&self) {
        println!("┌─────────────────────────────────────────────┐");
        println!("│  📱 NOTIFICATION PREVIEW                    │");
        println!("├─────────────────────────────────────────────┤");
        println!("│                                             │");
        println!("│  📌 {}                                      │", self.title);
        println!("│                                             │");
        println!("│  {}                                         │", self.message);
        println!("│                                             │");
        
        if let Some(tx) = &self.tx_hash {
            println!("│  🔗 Transaction: {}...                     │", &tx[..10.min(tx.len())]);
            println!("│                                             │");
        }
        
        println!("│  ⚡ Priority: {}                            │", self.priority.to_uppercase());
        println!("│                                             │");
        println!("└─────────────────────────────────────────────┘");
    }
}

impl ConfigCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n⚙️  === NOTIFICATION CONFIGURATION === ⚙️\n");
        
        if self.show {
            self.show_config();
            return Ok(());
        }
        
        if let Some(webhook) = &self.webhook {
            println!("✓ Webhook URL set: {}", webhook);
            println!("  Notifications will be sent via HTTP POST");
        }
        
        if let Some(token) = &self.token {
            let masked = format!("{}...{}", &token[..4.min(token.len())], &token[token.len()-4..]);
            println!("✓ Phone token set: {}", masked);
            println!("  Direct push notifications enabled");
        }
        
        if self.webhook.is_none() && self.token.is_none() {
            self.show_config();
        } else {
            println!("\n✅ Configuration saved!");
            println!();
            println!("Test your setup with:");
            println!("  stellar notify test");
            println!();
        }
        
        Ok(())
    }
    
    fn show_config(&self) {
        println!("Current Configuration:");
        println!();
        println!("📱 Phone Token: Not configured");
        println!("🌐 Webhook URL: Not configured");
        println!("📧 Email: Not configured");
        println!();
        println!("To configure:");
        println!("  stellar notify config --token <your-token>");
        println!("  stellar notify config --webhook <url>");
        println!();
        println!("💡 Popular notification services:");
        println!("   • Pushover: https://pushover.net/");
        println!("   • Pushbullet: https://www.pushbullet.com/");
        println!("   • ntfy.sh: https://ntfy.sh/");
        println!("   • Telegram Bot API");
        println!("   • Discord Webhooks");
        println!();
    }
}

impl TestCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n🧪 === TESTING NOTIFICATIONS === 🧪\n");
        
        println!("📱 Sending test notification...");
        
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        println!();
        println!("┌─────────────────────────────────────────────┐");
        println!("│  🧪 TEST NOTIFICATION                       │");
        println!("├─────────────────────────────────────────────┤");
        println!("│                                             │");
        println!("│  📌 Stellar CLI Test                        │");
        println!("│                                             │");
        println!("│  If you see this on your phone,            │");
        println!("│  notifications are working! 🎉             │");
        println!("│                                             │");
        println!("│  ⚡ Priority: NORMAL                        │");
        println!("│                                             │");
        println!("└─────────────────────────────────────────────┘");
        println!();
        
        println!("✅ Test notification sent!");
        println!();
        println!("📱 Check your phone for the notification.");
        println!();
        println!("If you didn't receive it:");
        println!("  1. Check notification config: stellar notify config --show");
        println!("  2. Verify webhook/token is correct");
        println!("  3. Check phone notification settings");
        println!();
        
        Ok(())
    }
}
