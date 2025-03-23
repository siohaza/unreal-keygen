use clap::{Parser, ValueEnum};
use unreal_keygen_core::{generate_key, GameVersion};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Game {
    UT2003,
    UT2004,
    UT3,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    game: Game,

    #[arg(short, long, default_value_t = 1)]
    count: u32,
}

fn main() {
    let cli = Cli::parse();

    let version = match cli.game {
        Game::UT2003 => GameVersion::UT2003,
        Game::UT2004 => GameVersion::UT2004,
        Game::UT3 => GameVersion::UT3,
    };

    for i in 0..cli.count {
        let key = generate_key(version);
        if cli.count > 1 {
            println!("Key {}: {}", i + 1, key);
        } else {
            println!("{}", key);
        }
    }
}
