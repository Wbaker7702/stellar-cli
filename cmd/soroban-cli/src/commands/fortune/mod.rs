use clap::Parser;
use rand::seq::SliceRandom;

#[derive(Parser, Debug)]
pub struct Cmd {
    /// Theme for the fortune (blockchain, stellar, moon,ламбо)
    #[arg(long, short = 't')]
    pub theme: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fortune telling failed")]
    FortuneTellingFailed,
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        let fortunes = match self.theme.as_deref() {
            Some("blockchain") => vec![
                "🔗 Your smart contract will execute flawlessly on the first try... said no one ever.",
                "⛓️ A bug-free deployment awaits you... after the 47th attempt.",
                "💎 Your next transaction will have such low fees, you'll think it's a bug.",
                "🚀 The blockchain will sync faster than your coffee brews today.",
                "📈 Your portfolio will moon... eventually. Maybe. Probably not.",
            ],
            Some("stellar") => vec![
                "⭐ Your XLM will reach for the stars, literally.",
                "🌟 A lumens windfall approaches from the cosmic void.",
                "✨ Your smart contracts will shine brighter than Sirius.",
                "🔭 The stellar network sees great things in your future.",
                "🌠 Your next deployment will be... astronomically successful!",
            ],
            Some("moon") => vec![
                "🌕 To the moon! (Disclaimer: Moon location may vary)",
                "🚀 Wen moon? Soon moon! (Not financial advice)",
                "🌙 Your bags are packed for the moon trip. Departure: TBD",
                "🧑‍🚀 Houston, we have liftoff! Destination: Moon. ETA: Unknown",
                "🌛 The moon whispers: 'HODL, young padawan'",
            ],
            Some("lambo") | Some("ламбо") => vec![
                "🏎️ Your lambo is in the mail... metaphorically speaking.",
                "🚗💨 Green candles lead to green lambos. Science fact.",
                "🏁 From ramen to lambo: a crypto journey in 1000 easy steps!",
                "🎮 Achievement unlocked: Imagined owning a lambo! 1/1000000",
                "🔧 Your lambo awaits... in the garage of your dreams.",
            ],
            _ => vec![
                "🔮 The blockchain oracle has spoken: DYOR!",
                "🎲 Your gas fees will be... unpredictable as always.",
                "🎭 In code we trust, in tests we verify.",
                "🧙 A wizard never deploys to mainnet without testing.",
                "🎪 Life is a circus, debugging is the tightrope walk.",
                "🦄 May your builds be swift and your deployments unicorn-rare in perfection.",
                "🐛 99 little bugs in the code, 99 bugs in the code. Take one down, patch it around, 117 little bugs in the code.",
                "☕ Warning: May contain traces of caffeine and existential dread.",
                "🎯 Your next commit will be clean... after 15 WIP attempts.",
                "🌈 At the end of the build log rainbow: either treasure or terror.",
            ],
        };

        let mut rng = rand::thread_rng();
        let fortune = fortunes.choose(&mut rng).ok_or(Error::FortuneTellingFailed)?;
        
        println!("\n╔═══════════════════════════════════════════════════════════════╗");
        println!("║         🔮 STELLAR BLOCKCHAIN FORTUNE TELLER 🔮              ║");
        println!("╚═══════════════════════════════════════════════════════════════╝\n");
        println!("  {}\n", fortune);
        
        Ok(())
    }
}
