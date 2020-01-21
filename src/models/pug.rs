use crate::models::player::Player;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pug
{
	pub identifier: String,
	pub description: Option<String>,
	pub max_players: usize,
	pub players: Vec<Player>,
	pub temporary: bool
}

impl Pug
{
	pub fn new_from_command(command: &Vec<&str>, temporary: bool) -> Option<Pug>
	{
		if command.len() < 2
		{
			return None;
		}

		let identifier = command[0].to_string();

		let max_players = match command[1].parse::<usize>()
		{
			Ok(x) => x,
			Err(_) => return None
		};

		let description = match command.len() == 3
		{
			true => Some(command[2].to_string()),
			false => None
		};

		Some(Pug
		{
            identifier: identifier,
            max_players: max_players,
            temporary: temporary,
            description: description,
            players: Vec::new()
		})
	}
}