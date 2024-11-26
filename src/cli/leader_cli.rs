use super::CRUDActions;
use super::StructOpt;

#[derive(Debug, StructOpt)]
pub struct LeadersCli {
    #[structopt(subcommand)]
    pub action: LeaderActions,
}

#[derive(Debug, StructOpt)]
pub enum LeaderActions {
    #[structopt(flatten)] // Combina variantes de CRUDActions
    Crud(CRUDActions<CreateLeader>),
}

#[derive(Debug, StructOpt)]
pub struct CreateLeader {
    pub name: String,
    pub personality: String,
}
