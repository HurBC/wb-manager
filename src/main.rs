use anyhow::{anyhow, Ok};
use cli::kingdom_cli::{
    CreateKingdom, KingdomLeadersOptions, KingdomTownsOptions, KingdomsAction, KingdomsCli,
    UpdateKingdom,
};
use cli::leader_cli::{CreateLeader, LeaderActions, LeadersCli, UpdateLeader};
use cli::town_cli::{CreateTown, TownsAction, TownsCli, UpdateTown};
use cli::{Cli, SubCommand};
use objects::leader::Leader;
use services::leader_services::delete_leader;
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
        SubCommand::Leaders(actions) => check_leaders_commands(actions, game_directory),
    }?;

    Ok(())
}

fn check_kingdoms_commands(action: KingdomsCli, directory: String) -> anyhow::Result<()> {
    let KingdomsCli { action } = action;

    match action {
        KingdomsAction::GetTowns => println!("get towns"),
        KingdomsAction::AddTown(KingdomTownsOptions {
            kingdom_index,
            town_index,
        }) => {
            services::kingdom_services::add_town_to_kingdom(town_index, kingdom_index, directory)?
        }
        KingdomsAction::AddTowns {
            kingdom_index,
            towns_index,
        } => {
            services::kingdom_services::add_towns_to_kingdom(towns_index, kingdom_index, directory)?
        }
        KingdomsAction::DeleteTown(KingdomTownsOptions {
            kingdom_index,
            town_index,
        }) => services::kingdom_services::delete_town_from_kingdom(
            town_index,
            kingdom_index,
            directory,
        )?,
        KingdomsAction::AddLeader(KingdomLeadersOptions {
            leader_index,
            kingdom_index,
        }) => services::kingdom_services::add_leader_to_kingdom(
            leader_index,
            kingdom_index,
            directory,
        )?,
        KingdomsAction::Crud(actions) => match actions {
            cli::CRUDActions::List => services::kingdom_services::list_kingdoms(directory)?,
            cli::CRUDActions::Create(CreateKingdom { name }) => {
                services::kingdom_services::add_kingdom(
                    objects::kingdom::Kingdom::new(name),
                    directory,
                )?
            }
            cli::CRUDActions::Delete { index } => {
                services::kingdom_services::delete_kingdom(index, directory)?
            }
            cli::CRUDActions::Update(UpdateKingdom { index, army, name }) => {
                todo!("update kingdom {} {} {}", army, name, index)
            }
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
            cli::CRUDActions::Delete { index } => {
                services::towns_services::delete_town(index, directory)?
            }
            cli::CRUDActions::Update(UpdateTown {
                index,
                name,
                population,
                loyalty,
            }) => println!("update town {} {} {}", name, population, loyalty),
            cli::CRUDActions::Get { index } => {
                services::towns_services::get_town(index, directory)?
            }
        },
    }

    Ok(())
}

fn check_leaders_commands(action: LeadersCli, directory: String) -> anyhow::Result<()> {
    let LeadersCli { action } = action;

    match action {
        LeaderActions::Crud(actions) => match actions {
            cli::CRUDActions::List => services::towns_services::list_towns(directory)?,
            cli::CRUDActions::Create(CreateLeader { name, personality }) => {
                services::leader_services::add_leader(Leader::new(name, personality), directory)?
            }
            cli::CRUDActions::Delete { index } => delete_leader(index, directory)?,
            cli::CRUDActions::Update(UpdateLeader { index, is_alive }) => {
                println!("update leader {}", is_alive)
            }
            cli::CRUDActions::Get { index } => todo!("Create leader function {}", index),
        },
    }

    Ok(())
}
