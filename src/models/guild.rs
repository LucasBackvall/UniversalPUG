use std::collections::HashMap;

use itertools::sorted;
use serde::{Serialize, Deserialize};
use serenity::model::id::UserId;
use serenity::client::Context;

use crate::models::player::Player;
use crate::models::pug::Pug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild
{
    pub identifier: u64,
    pub prefix: char,
    pub last_game: Option<String>,
    pub promote: bool,
    pub kick_offline: bool,
    pub pugs: HashMap<String, Pug>
}

impl Guild
{
    pub fn new(identifier: u64) -> Guild {
        Guild {
            identifier: identifier,
            prefix: '.',
            last_game: None,
            promote: false,
            kick_offline: true,
            pugs: HashMap::new()
        }
    }

    pub fn leave_all(&mut self, ctx: &Context, player: Player) -> Option<String>
    {
        let user_id = UserId(player.id);
        let mut message = match user_id.to_user(&ctx.http)
        {
            Ok(user) => user.name + " ",
            Err(e) => {
                println!("Error fetching User: {:?}", e);
                "".to_string()
            }
        } + "left pugs:";

        let mut send = false;
        for (identifier, mut pug) in &mut self.pugs
        {
            if player.leave_pug(&mut pug)
            {
                send = true;
                message += &(" ".to_owned() + identifier)
            }
        }

        if send {
            Some(message)
        }
        else {
            None
        }
    }

    pub fn list_pugs(&self, ctx: &Context) -> String
    {
        let rows: Vec<String> = sorted(
            self.pugs
                .clone()
                .iter()
                .map(|(identifier, pug)|
                {
                    let mut message = 
                        identifier.to_owned()
                        + ": (" + &pug.players.len().to_string() + "/" + &pug.max_players.to_string() + ")";

                    message += match &pug.description {
                        Some(desc) => &desc,
                        None => ""
                    };

                    if pug.players.len() > 0
                    {
                        message += " - "
                    }

                    for player in &pug.players
                    {
                        message += &(player.name(&ctx) + " ")
                    }

                    message
                }
            )
        ).collect();


        if rows.len() == 0
        {
            return "No pugs available. ".to_owned()
                + "Create a new one with "
                + "`.temp [pug name] [number of players]`";
        }

        let mut message = "```".to_string();
        for row in rows
        {
            message.push_str("\n");
            message.push_str(&row);
        }
        message.push_str("\n```");

        message
    }

    pub fn remove_temporary(&mut self) {
        let pugs = self.pugs.clone();
        for (identifier, pug) in pugs
        {
            if pug.temporary && pug.players.len() == 0
            {
                self.pugs.remove(&identifier);
            }
        }
    }
}
