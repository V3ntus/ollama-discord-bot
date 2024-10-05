use ollama_rs::Ollama;

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
