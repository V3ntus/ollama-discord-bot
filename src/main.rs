use colog;
use log::info;
use poise::serenity_prelude as serenity;

mod commands;
mod structs;
mod utils;

#[tokio::main]
async fn main() {
    colog::init();
    info!("Building bot framework and connecting to Discord...");
    
    let mut commands = vec![];
    commands.extend(commands::models::commands());

    serenity::ClientBuilder::new(
        std::env::var("DISCORD_SECRET").expect("DISCORD_SECRET environment variable not set"),
        serenity::GatewayIntents::non_privileged(),
    )
    .framework(
        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands,
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    let _guild_ids: [u64; 2] = [726156212457963601, 948337688824643594];
                    for _guild_id in &_guild_ids {
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            serenity::model::id::GuildId::new(*_guild_id),
                        )
                        .await?;
                        info!("Registered commands for guild {}", _guild_id);
                    }
                    Ok(structs::Data {})
                })
            })
            .build(),
    )
    .await
    .unwrap()
    .start()
    .await
    .unwrap();
}
