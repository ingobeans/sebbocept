use home;
use reqwest;
use std::env;
use std::fs::write;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("hello! wrong args :<");
        return;
    }
    let mut output_path = home::home_dir().unwrap();
    output_path.push("intercepted.seb");

    let data = &args[1];
    let mut stripped_text = "";
    let mut has_prefix = false;

    if let Some(stripped) = data.strip_prefix("sebs://") {
        has_prefix = true;
        stripped_text = stripped;
    }
    if let Some(stripped) = data.strip_prefix("seb://") {
        has_prefix = true;
        stripped_text = stripped;
    }

    if has_prefix {
        let url = "https://".to_string() + stripped_text;
        println!("hello! sending get request to {url} :>");
        let request = reqwest::get(url).await.unwrap();
        let body = request.text().await.unwrap();
        println!("got response yippie");
        write(&output_path, body.as_bytes()).unwrap();
        println!("wrote to {}", output_path.to_str().unwrap());
    } else {
        println!("expected sebs:// or seb://, not... whatever that is");
        return;
    }
}
