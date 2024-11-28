use structopt::StructOpt;

pub mod kingdom_cli;
pub mod leader_cli;
pub mod town_cli;

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
    Kingdoms(kingdom_cli::KingdomsCli),
    /// Towns commands
    Towns(town_cli::TownsCli),
    /// Leaders commands
    Leaders(leader_cli::LeadersCli),
}

#[derive(Debug, StructOpt)]
pub enum CRUDActions<TCreate: StructOpt, TUpdate: StructOpt> {
    /// List all objects
    List,
    /// Create a new object
    Create(TCreate),
    /// Delete an object
    Delete { index: usize },
    /// Update an object
    Update(TUpdate),
    /// Get an object
    Get { index: usize },
}
