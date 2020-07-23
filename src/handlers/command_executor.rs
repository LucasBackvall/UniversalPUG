use std::env;

use crate::models::guild::Guild;

use serenity::model::guild::GuildContainer;
use serenity::model::channel::Message;
use serenity::prelude::*;

pub struct CommandExecutor
{
	pub ctx: Context,
	pub msg: Message,
	pub guild: Guild
}

impl CommandExecutor
{
	pub fn execute_command(
		&mut self,
		mut command: Vec<&str>)
	{
		if command.len() == 0
		{
			return;
		}


        match command[0] {
        	"admin" | "adm" => self.admin_command(command.split_off(1)),
        	_ => self.pug_command(command)
        }

        match self.msg.delete(&self.ctx.http)
        {
        	Ok(()) => (),
        	Err(_e) => ()
        }
    }

    pub fn send_message(&self, message: String)
    {
        if let Err(e) = self.msg.channel_id.say(&self.ctx.http, message)
        {
            println!("Error sending message: {:?}", e);
        }
    }

    pub fn reply(&self, message: String)
    {
        if let Err(e) = self.msg.author.direct_message(
            &self.ctx.http, |m| m.content(message))
        {
            println!("Error sending reply: {:?}", e);
        }
    }

    pub fn list_pugs(&self, message: String)
    {
        let m = message + "\n" + &self.guild.list_pugs(&self.ctx);
        self.send_message(m);
    }

    pub fn is_admin(&self) -> bool
    {
        let admin_role = env::var("ADMIN_ROLE")
            .expect("Expected a admin role in the environment");

        match self.msg.guild(&self.ctx.cache)
        {
            Some(g) => match g.read().role_by_name(&admin_role)
            {
                Some(r) => match self.msg.author.has_role(
                    &self.ctx, GuildContainer::from(g.read().id), r)
                    {
                        Ok(res) => res,
                        Err(_e) => {
                            println!("Could not check if user has role");
                            false
                        }
                    },
                None => false
            },
            None => false
        }

    }
}
