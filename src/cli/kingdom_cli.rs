use super::StructOpt;
use super::CRUDActions;

#[derive(Debug, StructOpt)]
pub struct KingdomsCli {
    #[structopt(subcommand)]
    pub action: KingdomsAction,
}

// Actions
#[derive(Debug, StructOpt)]
pub enum KingdomsAction {
    // Kingdom CRUD
    #[structopt(flatten)]
    Crud(CRUDActions<CreateKingdom>),
    /// List all towns of a kingdom
    GetTowns,
    /// Add a new town to a kingdom
    AddTown {
        /// Index of the Town
        #[structopt(short, long)]
        town_index: usize,
        /// ID of the Kingdom
        #[structopt(short, long)]
        kingdom_index: usize,
    },
    /// Add multiple towns to a kingdom
    AddTowns {
        /// Index of the Kingdom
        #[structopt(short, long, required = true)]
        kingdom_index: usize,
        /// ID's of the Towns
        #[structopt(short, long, required = true)]
        towns_index: Vec<usize>,
    },
    /// Delete a town from a kingdom
    DeleteTown {
        /// Index of the Town
        #[structopt(short, long)]
        town_index: usize,
        /// ID of the Kingdom
        #[structopt(short, long)]
        kingdom_index: usize,
    },
}

#[derive(Debug, StructOpt)]
pub struct CreateKingdom {
    pub name: String,
}
