use crate::models::pug::Pug;
use serenity::model::id::UserId;
use serenity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player
{
	pub id: u64
}

impl PartialEq for Player
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

impl Player
{
	pub fn name(&self, ctx: &Context) -> String
	{
		let user_id = UserId(self.id);
		let user = match user_id.to_user(&ctx.http)
		{
			Ok(user) => user,
			Err(e) => {
				println!("Error fetching username: {:?}", e);
				return "".to_string()
			}
		};

		user.name
	}

	pub fn mention(&self, ctx: &Context) -> String
	{
		let user_id = UserId(self.id);
		let user = match user_id.to_user(&ctx.http)
		{
			Ok(user) => user,
			Err(e) => {
				println!("Error fetching username: {:?}", e);
				return "".to_string()
			}
		};

		user.mention()
	}

	pub fn direct_message(&self, ctx: &Context, message: String)
	{
		let user_id = UserId(self.id);
		let user = match user_id.to_user(&ctx.http)
		{
			Ok(user) => user,
			Err(e) => {
				println!("Error fetching username: {:?}", e);
				return ();
			}
		};

		if let Err(_e) = user.direct_message(&ctx.http, |m| m.content(message)) { }
	}

	pub fn leave_pug(&self, pug: &mut Pug) -> bool
	{
        if pug.players.contains(&self)
        {
            pug.players.retain(|p| p != self);
            return true
        }

        return false
	}
}