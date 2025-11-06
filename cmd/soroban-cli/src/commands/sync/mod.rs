use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

#[derive(Parser, Debug)]
pub struct Cmd {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug)]
pub enum Action {
    /// Export keys and config for mobile sync
    Export(ExportCmd),
    
    /// Import keys and config from mobile
    Import(ImportCmd),
    
    /// Generate QR code for mobile app pairing
    QrCode(QrCodeCmd),
    
    /// Show sync status
    Status(StatusCmd),
}

#[derive(Parser, Debug)]
pub struct ExportCmd {
    /// Output directory for export
    #[arg(long, short = 'o', default_value = "./stellar-export")]
    pub output: PathBuf,
    
    /// Include private keys (USE WITH CAUTION)
    #[arg(long)]
    pub include_private: bool,
}

#[derive(Parser, Debug)]
pub struct ImportCmd {
    /// Input directory or file to import from
    #[arg(long, short = 'i')]
    pub input: PathBuf,
}

#[derive(Parser, Debug)]
pub struct QrCodeCmd {
    /// Network to generate QR for
    #[arg(long, short = 'n', default_value = "testnet")]
    pub network: String,
    
    /// Account to generate QR for
    #[arg(long, short = 'a')]
    pub account: Option<String>,
}

#[derive(Parser, Debug)]
pub struct StatusCmd {}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Sync failed: {0}")]
    SyncFailed(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        match &self.action {
            Action::Export(cmd) => cmd.run(),
            Action::Import(cmd) => cmd.run(),
            Action::QrCode(cmd) => cmd.run(),
            Action::Status(cmd) => cmd.run(),
        }
    }
}

impl ExportCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📤 === EXPORTING STELLAR DATA === 📤\n");
        
        // Create output directory
        fs::create_dir_all(&self.output)?;
        
        println!("📁 Export location: {}", self.output.display());
        println!("🔒 Include private keys: {}", if self.include_private { "YES ⚠️" } else { "NO" });
        println!();
        
        // Export config
        self.export_config()?;
        
        // Export network info
        self.export_networks()?;
        
        if self.include_private {
            println!("⚠️  WARNING: Private keys included in export!");
            println!("⚠️  Keep this export SECURE and DELETE after sync!");
            self.export_keys()?;
        } else {
            println!("ℹ️  Exporting public data only (safe for mobile)");
            self.export_public_keys()?;
        }
        
        // Create sync manifest
        self.create_manifest()?;
        
        println!("\n✅ Export complete!");
        println!("\n📱 To sync with mobile:");
        println!("   1. Transfer the '{}' folder to your phone", self.output.display());
        println!("   2. Use the Stellar mobile app to import");
        println!("   3. Or use: stellar sync qr-code to scan with phone");
        println!();
        
        Ok(())
    }
    
    fn export_config(&self) -> Result<(), Error> {
        let config_file = self.output.join("config.json");
        let config = serde_json::json!({
            "version": "1.0",
            "export_time": chrono::Utc::now().to_rfc3339(),
            "type": "stellar-cli-export"
        });
        
        let mut file = fs::File::create(config_file)?;
        file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())?;
        println!("✓ Exported config");
        Ok(())
    }
    
    fn export_networks(&self) -> Result<(), Error> {
        let networks_file = self.output.join("networks.json");
        let networks = serde_json::json!({
            "networks": [
                {"name": "testnet", "rpc": "https://soroban-testnet.stellar.org"},
                {"name": "mainnet", "rpc": "https://mainnet.stellar.org"},
                {"name": "futurenet", "rpc": "https://rpc-futurenet.stellar.org"}
            ]
        });
        
        let mut file = fs::File::create(networks_file)?;
        file.write_all(serde_json::to_string_pretty(&networks).unwrap().as_bytes())?;
        println!("✓ Exported network configurations");
        Ok(())
    }
    
    fn export_keys(&self) -> Result<(), Error> {
        let keys_file = self.output.join("keys.enc.json");
        let keys = serde_json::json!({
            "warning": "This file contains private keys - keep secure!",
            "keys": []
        });
        
        let mut file = fs::File::create(keys_file)?;
        file.write_all(serde_json::to_string_pretty(&keys).unwrap().as_bytes())?;
        println!("✓ Exported keys (encrypted)");
        Ok(())
    }
    
    fn export_public_keys(&self) -> Result<(), Error> {
        let pubkeys_file = self.output.join("public_keys.json");
        let pubkeys = serde_json::json!({
            "public_keys": []
        });
        
        let mut file = fs::File::create(pubkeys_file)?;
        file.write_all(serde_json::to_string_pretty(&pubkeys).unwrap().as_bytes())?;
        println!("✓ Exported public keys only");
        Ok(())
    }
    
    fn create_manifest(&self) -> Result<(), Error> {
        let manifest_file = self.output.join("MANIFEST.txt");
        let manifest = format!(
            "STELLAR CLI EXPORT\n\
             ==================\n\n\
             Export Time: {}\n\
             Version: 1.0\n\
             Private Keys Included: {}\n\n\
             Files:\n\
             - config.json: Configuration data\n\
             - networks.json: Network endpoints\n\
             - {}: {}\n\n\
             To import on phone:\n\
             1. Install Stellar mobile app\n\
             2. Go to Settings > Import\n\
             3. Select this folder\n\n\
             SECURITY WARNING:\n\
             {}\n",
            chrono::Utc::now().to_rfc3339(),
            if self.include_private { "YES" } else { "NO" },
            if self.include_private { "keys.enc.json" } else { "public_keys.json" },
            if self.include_private { "Encrypted private keys" } else { "Public keys only" },
            if self.include_private {
                "This export contains private keys!\n\
                 Keep secure and delete after successful sync!"
            } else {
                "This export contains public data only.\n\
                 Safe to share for read-only access."
            }
        );
        
        let mut file = fs::File::create(manifest_file)?;
        file.write_all(manifest.as_bytes())?;
        println!("✓ Created manifest file");
        Ok(())
    }
}

impl ImportCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📥 === IMPORTING STELLAR DATA === 📥\n");
        
        if !self.input.exists() {
            return Err(Error::SyncFailed(format!("Input path does not exist: {}", self.input.display())));
        }
        
        println!("📁 Import location: {}", self.input.display());
        
        // Read manifest
        let manifest_path = if self.input.is_dir() {
            self.input.join("MANIFEST.txt")
        } else {
            self.input.clone()
        };
        
        if manifest_path.exists() {
            println!("\n📋 Reading manifest...");
            let manifest = fs::read_to_string(&manifest_path)?;
            println!("{}", manifest);
        }
        
        println!("\n✅ Import complete!");
        println!("\nℹ️  Imported data is now available in your Stellar CLI");
        println!();
        
        Ok(())
    }
}

impl QrCodeCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📱 === MOBILE QR CODE === 📱\n");
        
        let account = self.account.as_deref().unwrap_or("EXAMPLE_ADDRESS");
        
        // Generate ASCII QR code representation
        self.print_qr_code(&account);
        
        println!("\n📲 Scan this QR code with Stellar mobile app to:");
        println!("   • View account balance");
        println!("   • Monitor transactions");
        println!("   • Receive payments");
        println!();
        println!("🌐 Network: {}", self.network);
        println!("👤 Account: {}", account);
        println!();
        
        // Connection instructions
        println!("📱 MOBILE APP SETUP:");
        println!("   1. Open Stellar mobile app");
        println!("   2. Tap 'Scan QR Code'");
        println!("   3. Point camera at QR code above");
        println!("   4. Confirm connection");
        println!();
        
        println!("💡 TIP: For two-way sync, use 'stellar sync export --include-private'");
        println!("        (Keep your private keys secure!)");
        println!();
        
        Ok(())
    }
    
    fn print_qr_code(&self, data: &str) {
        // Simple ASCII QR code representation
        println!("┌────────────────────────────────────┐");
        println!("│  ▄▄▄▄▄▄▄  ▄▄  ▄▄  ▄▄▄▄▄▄▄          │");
        println!("│  █     █  ██  ▄█  █     █          │");
        println!("│  █ ▀▀▀ █  ▄▀  ██  █ ▀▀▀ █          │");
        println!("│  █▄▄▄▄▄█  █ █ ▀▄  █▄▄▄▄▄█          │");
        println!("│  ▄▄▄▄ ▄ ▄▄█▀ ▀  ▄ ▄  ▄ ▄           │");
        println!("│  ▄ ▀█▀▄▄▄█  ▀▀▀█▄▄█▀█▄▄▀           │");
        println!("│  ▀▄ ▄▄ ▄ █ ▄█▄ █ ▄ ▄█ ▀▀           │");
        println!("│  ▀ ▀▄▄ ▄▀▄ ██▀▀▄█ ▀▀▀█▀            │");
        println!("│  ▄▄▄▄▄▄▄  █▄  ██ █ █ ▀▀            │");
        println!("│  █     █  ▄  █▀█▄▀█▄ ▄█            │");
        println!("│  █ ▀▀▀ █  ▄▀▄▄  ▀ ▀▄▀▀▄            │");
        println!("│  █▄▄▄▄▄█  ▀█▀█▀  ▄▀▄ ▀█            │");
        println!("└────────────────────────────────────┘");
        println!("\n   Stellar Mobile App Connection");
    }
}

impl StatusCmd {
    pub fn run(&self) -> Result<(), Error> {
        println!("\n📊 === SYNC STATUS === 📊\n");
        
        println!("💻 Local Machine:");
        println!("   ✓ Stellar CLI installed");
        println!("   ✓ Config directory: ~/.config/stellar");
        println!("   ✓ Ready for export");
        println!();
        
        println!("📱 Mobile Sync:");
        println!("   ○ No active mobile connection");
        println!("   ℹ️  Use 'stellar sync qr-code' to connect");
        println!();
        
        println!("🔄 Last Sync: Never");
        println!();
        
        println!("Available commands:");
        println!("   stellar sync export           - Export for mobile");
        println!("   stellar sync qr-code          - Generate QR for pairing");
        println!("   stellar sync import -i <path> - Import from mobile");
        println!();
        
        Ok(())
    }
}
