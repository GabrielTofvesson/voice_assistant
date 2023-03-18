use elevenlabs_rs::{elevenlabs_api::ElevenLabsAPI, model::{tts::TTSMessage, voice::VoiceSettings}};
use openai_rs::{context::Context, chat::{ChatMessage, Role, ChatHistoryBuilder}, transcription::{TranscriptionRequestBuilder, AudioFile}, translation::TranslationRequestBuilder};
use tokio::{fs::File, io::AsyncWriteExt};

fn get_file(name: &str) -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(std::path::Path::new(name))?.trim().to_string())
}

fn get_openai() -> anyhow::Result<Context> {
    Ok(Context::new(get_file("openai.key")?))
}

fn get_elevenlabs() -> anyhow::Result<ElevenLabsAPI> {
    Ok(ElevenLabsAPI::new(get_file("elevenlabs.key")?))
}

const VOICE_ID: &str = "u339B6b9cariBZ7Vw3q4";
const INPUT_FILE: &str = "input_prompt.mp3";

async fn transform_prompt(openai: &Context, prompt: File) -> anyhow::Result<String> {
    Ok(openai.create_translation(TranslationRequestBuilder::default().prompt("[English]").model("whisper-1").file(AudioFile::MP3(prompt)).build()?).await?.text)
}

async fn generate_response(openai: &Context, elevenlabs: &ElevenLabsAPI, history: &mut Vec<ChatMessage>) {
    let response = openai.create_chat_completion(
        ChatHistoryBuilder::default()
            .messages(history.clone())
            .model("gpt-3.5-turbo")
            .build()
            .unwrap()
    ).await;

    if let Ok(mut response) = response {
        let response = response.choices.remove(0).message;
        let tts = elevenlabs.generate_tts(VOICE_ID.to_owned(), TTSMessage::new(response.content.clone(), VoiceSettings {
            stability: 0.5,
            similarity_boost: 0.75,
        }));
        history.push(response);

        let tts = tts.await;
        if let Ok(tts) = tts {
            File::create("response.wav").await.unwrap().write_all(tts.audio()[0].as_slice()).await.unwrap();
        } else {
            println!("{:?}", tts);
        }
    } else {
        println!("{:?}", response);
    }
    let response = openai.create_chat_completion(
        ChatHistoryBuilder::default()
            .messages(history.clone())
            .model("gpt-3.5-turbo")
            .build()
            .unwrap()
    ).await;

    if let Ok(mut response) = response {
        let response = response.choices.remove(0).message;
        let tts = elevenlabs.generate_tts(VOICE_ID.to_owned(), TTSMessage::new(response.content.clone(), VoiceSettings {
            stability: 0.5,
            similarity_boost: 0.75,
        }));
        history.push(response);

        let tts = tts.await;
        if let Ok(tts) = tts {
            File::create("response.wav").await.unwrap().write_all(tts.audio()[0].as_slice()).await.unwrap();
        } else {
            println!("{:?}", tts);
        }
    } else {
        println!("{:?}", response);
    }
}

#[tokio::main]
async fn main() {
    let openai = get_openai().unwrap();
    let elevenlabs = get_elevenlabs().unwrap();

    // Start of chat
    let mut history: Vec<ChatMessage> = Vec::new();
    history.push(ChatMessage::new(Role::System, "You are a voice assistant; Give helpful, accurate and concise responses. Your name is Jarvis. You are currently only capable of responding to prompts."));

    let response = transform_prompt(&openai, File::open(INPUT_FILE).await.unwrap()).await;

    if let Ok(response) = response {
        history.push(ChatMessage::new(Role::User, response));
        generate_response(&openai, &elevenlabs, &mut history).await;
    } else {
        println!("{:?}", response);
    }
}
