use elevenlabs_rs::elevenlabs_api::ElevenLabsAPI;
use openai_rs::context::Context;

fn get_file(name: &str) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(std::path::Path::new(name))?.trim().to_string())
}

fn get_openai() -> anyhow::Result<Context> {
    Ok(Context::new(get_file("openai.key")?))
}

fn get_elevenlabs() -> anyhow::Result<ElevenLabsAPI> {
    Ok(ElevenLabsAPI::new(get_file("elevenlabs.key")?))
}

fn main() {
    println!("Hello, world!");
}
