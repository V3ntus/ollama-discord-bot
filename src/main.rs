use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use tracing::info;

mod commands;
mod structs;
mod utils;

#[tokio::main]
async fn main() {
    // Load from .env
    dotenv().ok();

    // Install global tracer for log messages
    tracing_subscriber::fmt::init();

    info!("Building bot framework and connecting to Discord...");

    // Global commands vector
    let mut commands = vec![];
    commands.extend(commands::models::commands());
    commands.extend(commands::chat::commands());

    serenity::ClientBuilder::new(
        std::env::var("DISCORD_SECRET").expect("DISCORD_SECRET environment variable not set"),
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(
        poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands,
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    // TODO: register globally after development
                    // let _guild_ids: [u64; 2] = [726156212457963601, 948337688824643594];
                    let _guild_ids: [u64; 1] = [948337688824643594];
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
