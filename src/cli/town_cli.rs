use super::StructOpt;
use super::CRUDActions;

#[derive(Debug, StructOpt)]
pub struct TownsCli {
    #[structopt(subcommand)]
    pub action: TownsAction,
}

#[derive(Debug, StructOpt)]
pub enum TownsAction {
    #[structopt(flatten)] // Combina variantes de CRUDActions
    Crud(CRUDActions<CreateTown>),
}

#[derive(Debug, StructOpt)]
pub struct CreateTown {
    pub name: String,
    pub population: u32,
}