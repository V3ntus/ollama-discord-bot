use crate::{structs, utils};
use chrono::Utc;
use ollama_rs::generation::completion::{
    request::GenerationRequest,
    GenerationResponseStream,
};
use ollama_rs::Ollama;
use poise::futures_util::StreamExt;
use serenity::all::{AutoArchiveDuration, ChannelType, GuildId, Message};
use serenity::builder::{CreateMessage, CreateThread, EditMessage};
use tracing::{debug, info, warn};

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
    ephemeral,
    description_localized("en-US", "Create new chat session")
)]
async fn new(
    _ctx: structs::Context<'_>,
    #[description = "The model to use"]
    #[autocomplete = "utils::autocomplete_models"]
    model_name: String,
    #[description = "Your initial chat prompt to the LLM"] prompt: String,
    #[description = "Influence of predictability and creativity (0.8)"] temperature: Option<f32>,
    #[description = "Discourage repetitive or redundant output (1.1)"] frequency_penalty: Option<
        f32,
    >,
) -> Result<(), structs::Error> {
    _ctx.defer_ephemeral().await?;

    info!(
        "Got request to start a new chat. Model: {} - Prompt: \"{}\"",
        model_name, prompt
    );

    let mut _temperature: f32 = 0.8;
    let mut _frequency_penalty: f32 = 1.1;

    let ollama: Ollama = utils::create_ollama();

    // Temperature validation
    if let Some(temperature) = temperature {
        if !(0.0..=1.0).contains(&temperature) {
            _ctx.reply("Temperature must be between 0.0 and 1.0")
                .await?;
            return Ok(());
        }
        _temperature = temperature;
    }

    // Repeat penalty validation
    if let Some(frequency_penalty) = frequency_penalty {
        if !(0.0..=2.0).contains(&frequency_penalty) {
            _ctx.reply("Frequency penalty must be between 0.0 and 2.0")
                .await?;
            return Ok(());
        }
        _frequency_penalty = frequency_penalty;
    }

    debug!(
        "Chat session will be created with temperature {:?} and frequency_penalty {:?}",
        temperature, frequency_penalty,
    );

    match _ctx
        .guild_channel()
        .await
        .ok_or("Could not resolve guild channel")
    {
        Ok(channel) => {
            debug!("Creating thread in channel {}...", channel.id.to_string());
            let new_thread = channel
                .create_thread(
                    _ctx,
                    CreateThread::new("New chat")
                        .auto_archive_duration(AutoArchiveDuration::OneWeek)
                        .rate_limit_per_user(5)
                        .invitable(false)
                        .kind(ChannelType::PrivateThread),
                )
                .await?;

            debug!("Adding author to thread {}...", new_thread.id);
            _ctx.http()
                .add_thread_channel_member(new_thread.id, _ctx.author().id)
                .await?;
            _ctx.reply("Created thread").await?;

            debug!("Requesting Ollama to generate a stream with model and prompt...");
            let mut _edits_so_far = 0;
            let mut _response_so_far = String::new();
            let mut _message: Option<Message> = None;

            // TODO: this needs to either capture the stream context or use the chat function
            let mut stream: GenerationResponseStream = ollama
                .generate_stream(GenerationRequest::new(model_name, prompt))
                .await
                .unwrap();

            new_thread.broadcast_typing(_ctx).await?;

            while let Some(res) = stream.next().await {
                let responses = res.unwrap();

                for resp in responses {
                    _response_so_far.push_str(resp.response.as_str());

                    new_thread.broadcast_typing(_ctx).await?;

                    match _message {
                        Some(ref mut _message) => {
                            let duration_since_last_edit = Utc::now()
                                .signed_duration_since(_message.timestamp.to_utc())
                                .num_seconds();
                            if _edits_so_far <= duration_since_last_edit {
                                info!(
                                    "Editing message at thread {} with response chunk...",
                                    new_thread.id,
                                );
                                _message
                                    .edit(_ctx, EditMessage::new().content(&_response_so_far))
                                    .await?;
                                _edits_so_far += 1;
                            } else {
                                info!(
                                    "Exceeded ratelimit {} edits in {} seconds, \
                                    not editing for thread {}",
                                    _edits_so_far, duration_since_last_edit, new_thread.id,
                                );
                            }
                        }
                        None => {
                            _message = Some(
                                new_thread
                                    .send_message(
                                        _ctx,
                                        CreateMessage::new().content(&_response_so_far),
                                    )
                                    .await?,
                            );
                            info!("Sent initial response chunk");
                        }
                    }
                }
            }
            info!(
                "Done with chat request, sending/updating with final response for thread {}",
                new_thread.id,
            );
            match _message {
                Some(ref mut _message) => {
                    _message
                        .edit(_ctx, EditMessage::new().content(&_response_so_far))
                        .await?;
                }
                None => {
                    _message = Some(
                        new_thread
                            .send_message(_ctx, CreateMessage::new().content(&_response_so_far))
                            .await?,
                    );
                }
            }
        }
        Err(_err) => {
            warn!(
                "{}: Guild ID = {}",
                _err,
                _ctx.guild_id().unwrap_or(GuildId::new(1))
            );
            _ctx.reply(_err).await?;
            return Ok(());
        }
    }

    Ok(())
}

pub fn commands() -> Vec<poise::structs::Command<structs::Data, structs::Error>> {
    vec![chat()]
}
