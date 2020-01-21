use std::env;
use std::fs::{File};
use std::fs;
use std::io::{BufReader, Result, Error, ErrorKind};
use std::io::prelude::*;
use std::collections::HashMap;

use crate::models::guild::Guild;

pub struct GuildRepository
{
    guilds: HashMap<u64, Guild>
}

impl GuildRepository
{
	pub fn new() -> GuildRepository
	{
		let mut repo = GuildRepository
		{
			guilds: HashMap::new()
		};

	    match repo.init()
	    {
	        Ok(()) => repo,
	        Err(e) => panic!("Could not init guild repo: {:?}", e)
	    }
	}

	pub fn get(&mut self, guild_identifier: u64) -> Guild
	{
		self.ensure_guild(guild_identifier);
		match self.guilds.get(&guild_identifier) {
			Some(guild) => guild.clone(),
			None => panic!("Guild should be ensured, but it's not")
		}
	}

	pub fn put(&mut self, guild_identifier: u64, guild: Guild) -> Result<()>
	{
		self.ensure_guild(guild_identifier);
		self.guilds.insert(guild.identifier, guild);
		self.save_changes()
	}

	fn init(&mut self) -> Result<()>
	{
		match env::var("DATABASE_PATH") {
			Err(e) => Err(Error::new(ErrorKind::NotFound, e)),
			Ok(path) => {
				let file = match File::open(&path)
				{
					Ok(file) => file,
					Err(_e) => {
						fs::write(&path, "{}")?;
						File::open(&path)?
					}
				};
				let mut reader = BufReader::new(file);
				let mut content = String::new();
				reader.read_to_string(&mut content)?;

				self.guilds = serde_json::from_str(&content).unwrap();

				Ok(())
			}
		}
	}


	fn ensure_guild(&mut self, guild_identifier: u64) {
		let optional_guild = self.guilds.get(&guild_identifier);
		match optional_guild {
			Some(_guild) => (),
			None => {
				let guild = Guild::new(guild_identifier);
				self.guilds.insert(guild_identifier, guild);
			}
		}
	}

	fn save_changes(&self) -> Result<()> {
		match env::var("DATABASE_PATH") {
			Err(e) => Err(Error::new(ErrorKind::NotFound, e)),
			Ok(path) => {
				let content = serde_json::to_string_pretty(&self.guilds).unwrap();
				fs::write(path, content)?;

				Ok(())
			}
		}
	}
}