use super::StructOpt;
use super::kingdom_cli::KingdomsCli;
use super::town_cli::TownsCli;

#[derive(StructOpt, Debug)]
#[structopt(name = "wb-manager")]
pub struct Cli {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,

    /// Directory where the data is stored
    #[structopt(short, long)]
    pub game_directory: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    /// Kingdoms commands
    Kingdoms(KingdomsCli),
    /// Towns commands
    Towns(TownsCli),
}
