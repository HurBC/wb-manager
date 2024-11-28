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
    Crud(CRUDActions<CreateLeader, UpdateLeader>),
}

#[derive(Debug, StructOpt)]
pub struct CreateLeader {
    pub name: String,
    pub personality: String,
}

#[derive(Debug, StructOpt)]
pub struct UpdateLeader {
    #[structopt(short, long, name = "Leader index")]
    pub index: usize,
    #[structopt(long, name = "Kill the leader")]
    pub is_alive: String,
}
