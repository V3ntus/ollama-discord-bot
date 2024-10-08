use ollama_rs::{Ollama, models::LocalModel};
use crate::structs;
use poise::serenity_prelude::AutocompleteChoice;

//noinspection HttpUrlsUsage
pub fn create_ollama() -> Ollama {
    Ollama::new(
        format!(
            "http://{}",
            std::env::var("OLLAMA_HOST").expect("OLLAMA_HOST environment variable missing")
        ),
        str::parse::<u16>(
            &*std::env::var("OLLAMA_PORT").expect("OLLAMA_PORT environment variable missing"),
        )
        .expect("Could not parse OLLAMA_PORT as integer"),
    )
}

/// Autocomplete callback for list of model names.
pub async fn autocomplete_models<'a>(
    _ctx: structs::Context<'_>,
    partial: &'a str,
) -> impl Iterator<Item = AutocompleteChoice> + 'a {
    let ollama: Ollama = create_ollama();
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

