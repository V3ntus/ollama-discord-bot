use crate::{structs, utils};
use ollama_rs::models::{LocalModel, ModelInfo};
use ollama_rs::Ollama;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor};
use serenity::all::AutocompleteChoice;

#[poise::command(
    slash_command,
    subcommands("list", "info"),
    description_localized("en-US", "Manage and list models available in Ollama.")
)]
async fn models(_ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "Get info on a specific model")
)]
async fn info(
    _ctx: structs::Context<'_>,
    #[description = "The model to request"]
    #[autocomplete = "autocomplete_models"]
    model_name: String,
) -> Result<(), structs::Error> {
    let _model_name = &model_name;
    let ollama: Ollama = utils::create_ollama();
    let model_info: Option<ModelInfo> = match ollama.show_model_info(model_name.clone()).await {
        Ok(model_info) => Some(model_info),
        Err(_) => {
            _ctx.reply(format!("Could not fetch info for model: `{model_name}`"))
                .await?;
            None
        }
    };

    if model_info.is_some() {
        let _model_info = model_info.unwrap();
        _ctx.send(
            poise::CreateReply::default().embed(
                CreateEmbed::new()
                    .title(_model_name)
                    .url(format!("https://ollama.com/library/{_model_name}"))
                    .author(CreateEmbedAuthor::new("Ollama")),
            ),
        )
        .await?;
    }
    Ok(())
}

#[poise::command(
    slash_command,
    description_localized("en-US", "List all models available in Ollama.")
)]
async fn list(_ctx: structs::Context<'_>) -> Result<(), structs::Error> {
    let ollama: Ollama = utils::create_ollama();
    let model_names = ollama
        .list_local_models()
        .await
        .unwrap()
        .iter()
        .map(|m| format!("`{}`", m.name))
        .collect::<Vec<String>>()
        .join("\n");

    _ctx.reply(format!("## Models available:\n{model_names}"))
        .await?;
    Ok(())
}

/// Autocomplete callback for list of model names.
async fn autocomplete_models<'a>(
    _ctx: structs::Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + 'a {
    let ollama: Ollama = utils::create_ollama();
    let model_names: Vec<String> = ollama
        .list_local_models()
        .await
        .unwrap_or(vec![LocalModel {
            name: "Could not fetch models".to_string(),
            modified_at: "".to_string(),
            size: 0,
        }])
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<String>>();
    model_names
        .iter()
        .filter(|model_name| model_name.to_string().contains(partial))
        .map(|model_name| AutocompleteChoice::new(model_name, model_name.to_string()))
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn commands() -> Vec<poise::structs::Command<structs::Data, structs::Error>> {
    vec![models()]
}
