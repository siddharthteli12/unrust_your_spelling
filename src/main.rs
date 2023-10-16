use dotenv::dotenv;
use std::io::Cursor;
fn main() {
    dotenv().ok();
    let url = std::env::var("DICTIONARY_URL").unwrap();
    fetch_audio(url);
}

fn play_audio(_url: String) {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let response = reqwest::blocking::get(
        "https://api.dictionaryapi.dev/media/pronunciations/en/hello-au.mp3",
    )
    .unwrap();
    let cursor = Cursor::new(response.bytes().unwrap());
    let source = rodio::Decoder::new(cursor).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}
