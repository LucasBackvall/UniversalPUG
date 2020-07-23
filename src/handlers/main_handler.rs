use crate::models::player::Player;
use crate::handlers::command_executor::CommandExecutor;
use crate::repositories::guild_repository::GuildRepository;

use serenity::model::id::GuildId;
use serenity::model::event::PresenceUpdateEvent;
use serenity::model::user::OnlineStatus::*;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct MainHandler;

impl EventHandler for MainHandler
{
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message)
    {
        if msg.is_own(&ctx)
        {
            return;
        }

    	let mut guild_repo = GuildRepository::new();
        let guild_id = match msg.guild_id
        {
            Some(guild_id) => *guild_id.as_u64(),
            None => {
            	println!("Could not identify guild for message: {:?}", msg.content);
            	return;
            }
        };
        let guild = guild_repo.get(guild_id);

        if msg.content.len() <= 0 || msg.content.chars().next().unwrap() != guild.prefix
        {
        	return;
        }

        let message = msg.content.clone().split_off(1);
        if message.len() <= 0
        {
        	return;
        }

        let command: Vec<&str> = message.split(' ').collect();

        let mut command_executor = CommandExecutor
        {
        	ctx: ctx,
        	msg: msg,
        	guild: guild
        };

        command_executor.execute_command(command);

        match guild_repo.put(guild_id, command_executor.guild)
        {
        	Ok(()) => (),
        	Err(e) => println!("Error saving guild: {:?}", e)
        }
    }

    fn presence_update(&self, ctx: Context, event: PresenceUpdateEvent)
    {
        let mut guild_repo = GuildRepository::new();
        let guild_id = match event.guild_id
        {
            Some(guild_id) => *guild_id.as_u64(),
            None => {
                println!("Could not identify guild");
                return;
            }
        };
        let mut guild = guild_repo.get(guild_id);

        let presence = event.presence;

        if guild.kick_offline
            && (presence.status == Offline || presence.status == Invisible)
        {
            let player = Player
            {
                id: *presence.user_id.as_u64()
            };

            let mut message = match guild.leave_all(&ctx, player)
            {
                Some(message) => message,
                None => return
            };

            println!("Kick offline. Presence: {:?}", presence);

            guild.remove_temporary();
            message += &("\n".to_owned() + &guild.list_pugs(&ctx));

            let channels = GuildId(guild_id).channels(&ctx);

            match channels
            {
                Ok(channels) => for (channel_id, _channel) in channels
                    {
                        if let Err(_e) = channel_id.say(&ctx.http, &message) { }
                    },
                Err(e) => println!("Error fetching guild channels{:?}", e)
            }
        }

        match guild_repo.put(guild_id, guild)
        {
            Ok(()) => (),
            Err(e) => println!("Error saving guild: {:?}", e)
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
