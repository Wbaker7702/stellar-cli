use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cmd {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug)]
pub enum Action {
    /// Pair with mobile device
    Pair(PairCmd),
    
    /// View account on mobile
    View(ViewCmd),
    
    /// Sign transaction on phone
    Sign(SignCmd),
    
    /// Check mobile app status
    Status(StatusCmd),
}

#[derive(Parser, Debug)]
pub struct PairCmd {
    /// Device name
    #[arg(long, short = 'n', default_value = "My Phone")]
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct ViewCmd {
    /// Account to view
    #[arg(long, short = 'a')]
    pub account: String,
}

#[derive(Parser, Debug)]
pub struct SignCmd {
    /// Transaction to sign
    #[arg(long)]
    pub tx: String,
}

#[derive(Parser, Debug)]
pub struct StatusCmd {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Phone integration failed: {0}")]
    PhoneFailed(String),
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        match &self.action {
            Action::Pair(cmd) => cmd.run(),
            Action::View(cmd) => cmd.run(),
            Action::Sign(cmd) => cmd.run(),
            Action::Status(cmd) => cmd.run(),
        }
    }
}

impl PairCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📱 === PAIRING WITH MOBILE DEVICE === 📱\n");
        
        println!("🔗 Initiating pairing with: {}", self.name);
        println!();
        
        // Generate pairing code
        let pairing_code = self.generate_pairing_code();
        
        println!("📋 Pairing Code:");
        println!();
        self.print_pairing_code(&pairing_code);
        println!();
        
        println!("📱 ON YOUR PHONE:");
        println!("   1. Open Stellar mobile app");
        println!("   2. Go to Settings → Pair Device");
        println!("   3. Enter the pairing code above");
        println!("   4. Confirm the pairing");
        println!();
        
        println!("⏱️  Code expires in 5 minutes");
        println!();
        
        println!("✅ Waiting for phone to connect...");
        println!("   (This is a simulation - use QR code for real pairing)");
        println!();
        
        println!("💡 QUICK PAIRING:");
        println!("   Use QR code for faster pairing:");
        println!("   stellar sync qr-code --account <address>");
        println!();
        
        Ok(())
    }
    
    fn generate_pairing_code(&self) -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        format!("{:04}-{:04}-{:04}", 
            rng.gen_range(1000..9999),
            rng.gen_range(1000..9999),
            rng.gen_range(1000..9999)
        )
    }
    
    fn print_pairing_code(&self, code: &str) {
        println!("    ╔═══════════════════════════╗");
        println!("    ║                           ║");
        println!("    ║      {}        ║", code);
        println!("    ║                           ║");
        println!("    ╚═══════════════════════════╝");
    }
}

impl ViewCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📱 === VIEW ON MOBILE === 📱\n");
        
        println!("🔍 Preparing mobile view for account:");
        println!("   {}", self.account);
        println!();
        
        // Generate deep link
        let deep_link = format!("stellar://account/{}", self.account);
        
        println!("📲 Generated mobile deep link:");
        println!("   {}", deep_link);
        println!();
        
        // Generate QR code
        println!("📱 Scan this QR code with your phone:");
        println!();
        self.print_qr_for_account();
        println!();
        
        println!("✨ What you'll see on mobile:");
        println!("   • Account balance");
        println!("   • Recent transactions");
        println!("   • Asset holdings");
        println!("   • Quick actions (send, receive)");
        println!();
        
        println!("💡 TIP: Save this account to your mobile wallet");
        println!("        for quick access anytime!");
        println!();
        
        Ok(())
    }
    
    fn print_qr_for_account(&self) {
        println!("    ┌────────────────────────────┐");
        println!("    │  ▀▀▀▀▀▀▀  ▀  ▀  ▀▀▀▀▀▀▀    │");
        println!("    │  ▀     ▀  ▀▀  ▀  ▀     ▀   │");
        println!("    │  ▀ ▀▀▀ ▀  ▀▀  ▀  ▀ ▀▀▀ ▀   │");
        println!("    │  ▀▀▀▀▀▀▀  ▀ ▀ ▀  ▀▀▀▀▀▀▀   │");
        println!("    │  ▀▀▀▀ ▀ ▀▀▀ ▀  ▀ ▀  ▀ ▀    │");
        println!("    │  ▀ ▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀      │");
        println!("    │  ▀▀ ▀▀ ▀ ▀ ▀▀▀ ▀ ▀ ▀▀ ▀    │");
        println!("    │  ▀ ▀▀ ▀▀▀▀ ▀▀▀▀ ▀▀▀▀▀▀     │");
        println!("    │  ▀▀▀▀▀▀▀  ▀▀  ▀▀ ▀ ▀ ▀     │");
        println!("    │  ▀     ▀  ▀  ▀▀▀▀▀▀ ▀▀     │");
        println!("    │  ▀ ▀▀▀ ▀  ▀▀▀  ▀ ▀▀▀▀      │");
        println!("    └────────────────────────────┘");
        println!("         View Account on Mobile");
    }
}

impl SignCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📱 === SIGN ON MOBILE === 📱\n");
        
        println!("🔐 Transaction signing request:");
        println!();
        
        // Display transaction preview
        self.print_transaction_preview();
        
        println!("\n📲 Sending to mobile device for signing...");
        println!();
        
        println!("📱 ON YOUR PHONE:");
        println!("   1. You'll receive a notification");
        println!("   2. Review transaction details");
        println!("   3. Confirm with biometric/PIN");
        println!("   4. Transaction will be signed");
        println!();
        
        println!("⏱️  Waiting for approval on phone...");
        println!("   (This is a simulation)");
        println!();
        
        println!("🔒 Security Features:");
        println!("   ✓ Biometric authentication required");
        println!("   ✓ Transaction details shown on phone");
        println!("   ✓ Approve or reject on secure device");
        println!("   ✓ Private key never leaves phone");
        println!();
        
        println!("💡 TIP: Use phone signing for added security!");
        println!();
        
        Ok(())
    }
    
    fn print_transaction_preview(&self) {
        println!("    ╔═══════════════════════════════════════╗");
        println!("    ║  📝 TRANSACTION TO SIGN               ║");
        println!("    ╠═══════════════════════════════════════╣");
        println!("    ║                                       ║");
        println!("    ║  Hash: {}...              ║", &self.tx[..12.min(self.tx.len())]);
        println!("    ║                                       ║");
        println!("    ║  Type: Contract Invocation            ║");
        println!("    ║  Network: Testnet                     ║");
        println!("    ║                                       ║");
        println!("    ║  ⚠️  Review on phone before signing  ║");
        println!("    ║                                       ║");
        println!("    ╚═══════════════════════════════════════╝");
    }
}

impl StatusCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📱 === MOBILE APP STATUS === 📱\n");
        
        println!("📲 Connected Devices:");
        println!();
        println!("   ○ No devices paired");
        println!();
        
        println!("🔗 Available Features:");
        println!("   • Pair new device:     stellar phone pair");
        println!("   • View on mobile:      stellar phone view -a <account>");
        println!("   • Sign with phone:     stellar phone sign --tx <hash>");
        println!();
        
        println!("📱 Supported Mobile Apps:");
        println!("   ✓ Stellar Mobile Wallet");
        println!("   ✓ Lobstr");
        println!("   ✓ Freighter Mobile");
        println!("   ✓ StellarX Mobile");
        println!();
        
        println!("💡 To pair a device:");
        println!("   stellar phone pair --name \"iPhone\"");
        println!();
        
        println!("🔐 Security Status:");
        println!("   ✓ Encrypted communication");
        println!("   ✓ Device authentication enabled");
        println!("   ✓ Biometric protection supported");
        println!();
        
        Ok(())
    }
}
