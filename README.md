# Ollama Discord Bot

My first Rust project, dedicated to learning Rust, mainly to record my journey.
I'm gonna come back to this in a couple years and cringe at how many antipatterns I wrote.

- Using Serenity and Poise
- Uses ollama-rs to communicate to my Ollama instance

## Environment Variables
- `DISCORD_SECRET`
- `OLLAMA_HOST`
- `OLLAMA_PORT`

## TODO
- [X] Get model list and info (autocomplete)
- [X] **LLM chat mode:**
  - [ ] Session recording with Discord threads.
  - [X] Chat streaming by periodically editing reply.
  - [X] Ephemeral response when LLM generating.
- [ ] **LLM instruct mode:**
  - ???
