use crate::handlers::command_executor::CommandExecutor;
use crate::models::pug::Pug;

impl CommandExecutor
{
	pub fn admin_command(
		&mut self,
		mut command: Vec<&str>)
	{
		if command.len() == 0 || !self.is_admin()
		{
			return;
		}

		match command[0] {
			"pug" => {
        		if command.len() == 1
        		{
        			return;
        		}
        		match command[1] {
        			"new" => match Pug::new_from_command(&command.split_off(2), false)
                    {
                        Some(pug) => {
                            if self.guild.pugs.contains_key(&pug.identifier)
                            {
                                return;
                            }
                            self.guild.pugs.insert(
                                pug.identifier.clone(),
                                pug.clone()
                            );
                            self.list_pugs("Added new pug: ".to_string() + &pug.identifier);
                        },
                        None => ()
        			},
                    "del" => {
                        if command.len() == 2
                        {
                            return;
                        }
                        match self.guild.pugs.remove(command[2])
                        {
                            Some(key) => {
                                self.list_pugs(self.msg.author.name.to_string() + " removed pug " + &key.identifier);
                            }
                            None =>
                                self.reply("Pug not found.".to_string())
                        }
                    }
        			_ => ()
        		}
			},
			_ => ()
    	}
    }
}
