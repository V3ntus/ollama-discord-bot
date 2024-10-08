use ollama_rs::Ollama;
use serenity::model::channel::GuildChannel;

use crate::{structs, utils};

#[poise::command(
    slash_command,
    subcommands("new"),
    description_localized("en-US", "Create and manage chat sessions")
)]
async fn chat(_ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Create new chat session")
)]
async fn new(
    _ctx: structs::Context<'_>,
    #[description = "The model to use"]
    #[autocomplete = "utils::autocomplete_models"]
    model_name: String,
    #[description = "Influence of predictability and creativity (0.8)"]
    temperature: Option<f32>,
    #[description = "Discourage repetitive or redundant output (1.1)"]
    frequency_penalty: Option<f32>,
) -> Result<(), structs::Error> {
    _ctx.defer().await?;
    let _model_name = &model_name;
    let ollama: Ollama = utils::create_ollama();

    let channel: Result<GuildChannel, &str> = _ctx.guild_channel().await.ok_or("Could not resolve guild channel");
    
    Ok(())
}

