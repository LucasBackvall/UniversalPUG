use crate::models::pug::Pug;
use crate::models::player::Player;
use crate::handlers::command_executor::CommandExecutor;


impl CommandExecutor
{
	pub fn pug_command(
		&mut self,
		mut command: Vec<&str>)
	{
        if command.len() == 0
        {
            return;
        }
		match command[0] {
            // List all the pugs and their current player counts
			"list" | "ls" =>
            {
                let message = "Available pugs:".to_string();
                self.list_pugs(message);
			},
            // leave pug
            "leave" | "l" =>
            {
                if command.len() == 1
                {
                    return;
                }

                let identifier = command[1];

                match self.guild.pugs.get_mut(identifier)
                {
                    None => self.reply("Pug does not exist.".to_string()),
                    Some(pug) =>
                    {
                        let player = Player
                        {
                            id: *self.msg.author.id.as_u64()
                        };

                        if player.leave_pug(pug)
                        {
                            let message =
                                self.msg.author.name.to_string() + &" left ".to_string() + &identifier;

                            self.guild.remove_temporary();
                            self.list_pugs(message)
                        }
                        else
                        {
                            self.reply("Not in pug ".to_owned() + &identifier);
                        }
                    }
                };
            }

            "leaveall" | "lva" =>
            {
                let player = Player
                {
                    id: *self.msg.author.id.as_u64()
                };

                let message = match self.guild.leave_all(&self.ctx, player)
                {
                    Some(message) => message,
                    None => return
                };

                self.guild.remove_temporary();
                self.list_pugs(message)
            }

            // join pug
            "join" | "j" =>
            {
                if command.len() == 1
                {
                    return;
                }

                let mut message =
                    self.msg.author.name.to_string()
                    + " joined";

                let mut send = false;

                for pug_name in command.split_off(1)
                {
                    if self.join(pug_name)
                    {
                        send = true;
                        message += &(" ".to_owned() + pug_name);
                    }
                }

                self.check_full();

                if send
                {
                    self.guild.remove_temporary();
                    self.list_pugs(message);
                }
            }

            "temp" | "tmp" =>
            {
                if command.len() < 3
                {
                    return;
                }

                let pug_name = command[1];
                self.create_temp(command.split_off(1));

                if self.join(pug_name)
                {
                    self.check_full();
                    self.guild.remove_temporary();
                    self.list_pugs(
                        self.msg.author.name.to_string()
                        + " joined " + pug_name
                    );
                }
            }
			_ => ()
    	}
    }

    fn create_temp(&mut self, command: Vec<&str>)
    {
        if !self.guild.pugs.contains_key(command[0])
        {
            match Pug::new_from_command(&command, true)
            {
                Some(pug) => {
                    if self.guild.pugs.contains_key(&pug.identifier)
                    {
                        return;
                    }
                    self.guild.pugs.insert(
                        pug.identifier.clone(),
                        pug
                    );
                },
                None => self.pug_non_existing()
            };
        }
    }

    fn join(&mut self, command: &str) -> bool
    {
        match self.guild.pugs.get_mut(command)
        {
            None =>
            {
                self.pug_non_existing();
                return false;
            },
            Some(pug) =>
            {
                let player = Player
                {
                    id: *self.msg.author.id.as_u64()
                };

                if !pug.players.contains(&player)
                {
                    pug.players.push(player);
                    return true;
                }
                else
                {
                    let identifier = pug.identifier.to_string();
                    self.reply("Already in pug ".to_owned() + &identifier);
                    return false;
                }
            }
        };
    }

    fn check_full(&mut self)
    {
        for (identifier, pug) in self.guild.pugs.clone()
        {
            if pug.players.len() == pug.max_players
            {
                let players = pug.players.clone();
                let mut player_names = "\n".to_string();
                for player in &players
                {
                    player_names += &(player.mention(&self.ctx) + " ");
                }

                players
                    .iter()
                    .for_each(|player| {
                        player.direct_message(&self.ctx,
                            "Pug found: ".to_owned()
                            + &identifier
                            + &player_names
                        );
                        self.guild.leave_all(&self.ctx, player.clone());
                        return ();
                    });
                break;
            }
        }
    }

    fn pug_non_existing(&self)
    {
        self.reply(
            "Pug does not exist. To create a temporary pug type\n".to_owned()
            + "```\n"
            + "temp [pug name] [max player count] <description>\n"
            + "```"
        );
    }
}
