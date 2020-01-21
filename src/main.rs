extern crate dotenv;
extern crate serde;
 
mod models;
mod repositories;
mod handlers;

use crate::handlers::main_handler::MainHandler;

use dotenv::dotenv;
use std::env;

use serenity::prelude::*;

fn main() {
    dotenv().ok();

    // Configure the client with your Discord bot auth token from the .env file
    let token = env::var("AUTH_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, MainHandler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
