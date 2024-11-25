use anyhow::{anyhow, Ok};
use cli::cli::{SubCommand};
use cli::kingdom_cli::{KingdomsAction, KingdomsCli, CreateKingdom};
use cli::town_cli::{TownsAction, TownsCli, CreateTown};
use cli::cli::Cli;
use structopt::StructOpt;

mod cli;
mod objects;
mod services;
mod utils;

fn main() -> anyhow::Result<()> {
    let Cli {
        subcommand,
        game_directory,
    } = Cli::from_args();

    let game_directory = game_directory
        .or_else(utils::generate_random_string)
        .ok_or(anyhow!("Game directory not found"))?;

    match subcommand {
        SubCommand::Kingdoms(actions) => check_kingdoms_commands(actions, game_directory),
        SubCommand::Towns(actions) => check_towns_commands(actions, game_directory),
    }?;

    Ok(())
}

fn check_kingdoms_commands(action: KingdomsCli, directory: String) -> anyhow::Result<()> {
    let KingdomsCli { action } = action;

    match action {
        KingdomsAction::GetTowns => println!("get towns"),
        KingdomsAction::AddTown {
            kingdom_index,
            town_index,
        } => services::kingdom_services::add_town_to_kingdom(town_index, kingdom_index, directory)?,
        KingdomsAction::AddTowns {
            kingdom_index,
            towns_index,
        } => {
            services::kingdom_services::add_towns_to_kingdom(towns_index, kingdom_index, directory)?
        }
        KingdomsAction::DeleteTown {
            kingdom_index,
            town_index,
        } => services::kingdom_services::delete_town_from_kingdom(
            town_index,
            kingdom_index,
            directory,
        )?,
        KingdomsAction::Crud(actions) => match actions {
            cli::CRUDActions::List => services::kingdom_services::list_kingdoms(directory)?,
            cli::CRUDActions::Create(CreateKingdom { name }) => services::kingdom_services::add_kingdom(
                objects::kingdom::Kingdom::new(name),
                directory,
            )?,
            cli::CRUDActions::Delete { index } => {
                services::kingdom_services::delete_kingdom(index, directory)?
            }
            cli::CRUDActions::Update { index } => println!("update kingdom {}", index),
            cli::CRUDActions::Get { index } => {
                services::kingdom_services::get_kingdom(index, directory)?
            }
        },
    }

    Ok(())
}

fn check_towns_commands(action: TownsCli, directory: String) -> anyhow::Result<()> {
    let TownsCli { action } = action;

    match action {
        TownsAction::Crud(actions) => match actions {
            cli::CRUDActions::List => services::towns_services::list_towns(directory)?,
            cli::CRUDActions::Create(CreateTown { name, population }) => {
                let new_town = objects::towns::Town::new(name, population);

                services::towns_services::add_town(new_town, directory)?
            }
            cli::CRUDActions::Delete { index } => println!("delete town {}", index),
            cli::CRUDActions::Update { index } => println!("update town {}", index),
            cli::CRUDActions::Get { index } => services::towns_services::get_town(index, directory)?,
        },
    }

    Ok(())
}
