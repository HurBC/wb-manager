use super::CRUDActions;
use super::StructOpt;

#[derive(Debug, StructOpt)]
pub struct TownsCli {
    #[structopt(subcommand)]
    pub action: TownsAction,
}

#[derive(Debug, StructOpt)]
pub enum TownsAction {
    #[structopt(flatten)] // Combina variantes de CRUDActions
    Crud(CRUDActions<CreateTown, UpdateTown>),
}

#[derive(Debug, StructOpt)]
pub struct CreateTown {
    pub name: String,
    pub population: u32,
}

#[derive(Debug, StructOpt)]
pub struct UpdateTown {
    #[structopt(short, long, name = "Town index")]
    pub index: usize,
    #[structopt(short, long, name = "New name for the town")]
    pub name: String,
    #[structopt(short, long, name = "Update the town population")]
    pub population: u32,
    #[structopt(short, long, name = "Update the town kingdom loyalty")]
    pub loyalty: i32,
}
