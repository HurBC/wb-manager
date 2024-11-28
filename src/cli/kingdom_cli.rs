use super::CRUDActions;
use super::StructOpt;

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
    Crud(CRUDActions<CreateKingdom, UpdateKingdom>),
    /// List all towns of a kingdom
    GetTowns,
    /// Add a new town to a kingdom
    AddTown(KingdomTownsOptions),
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
    DeleteTown(KingdomTownsOptions),
    /// Add a leader from a kingdom
    AddLeader(KingdomLeadersOptions),
    // RemoveLeader {
    //     /// Index of the Leader
    //     #[structopt(short, long)]
    //     leader_index: usize,
    //     /// ID of the Kingdom
    //     #[structopt(short, long)]
    //     kingdom_index: usize,
    // }
}

#[derive(Debug, StructOpt)]
pub struct CreateKingdom {
    pub name: String,
}

#[derive(Debug, StructOpt)]
pub struct UpdateKingdom {
    #[structopt(short, long, name = "Kingdom index")]
    pub index: usize,
    #[structopt(short, long, name = "New name for the kingdom")]
    pub name: String,
    #[structopt(short, long, name = "Update the kingdom army")]
    pub army: usize,
}

#[derive(Debug, StructOpt)]
pub struct KingdomTownsOptions {
    #[structopt(short, long, name = "Index of the kingdom")]
    pub kingdom_index: usize,
    #[structopt(short, long, name = "Index of the towns")]
    pub town_index: usize,
}

#[derive(Debug, StructOpt)]
pub struct KingdomLeadersOptions {
    #[structopt(short, long, name = "Index of the kingdom")]
    pub kingdom_index: usize,
    #[structopt(short, long, name = "Index of the Leader")]
    pub leader_index: usize,
}
