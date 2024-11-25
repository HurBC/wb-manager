use structopt::StructOpt;

pub mod cli;
pub mod kingdom_cli;
pub mod town_cli;


#[derive(Debug, StructOpt)]
pub enum CRUDActions<TCreate: StructOpt> {
    /// List all objects
    List,
    /// Create a new object
    Create(TCreate),
    /// Delete an object
    Delete { index: usize },
    /// Update an object
    Update { index: usize },
    /// Get an object
    Get { index: usize },
}
