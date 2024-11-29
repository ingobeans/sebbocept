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
    let stripped = data.strip_prefix("sebs://");

    if let Some(stripped) = stripped {
        let url = "https://".to_string() + stripped;
        println!("hello! sending get request to {url} :>");
        let request = reqwest::get(url).await.unwrap();
        let body = request.text().await.unwrap();
        println!("got response yippie");
        write(&output_path, body.as_bytes()).unwrap();
        println!("wrote to {}", output_path.to_str().unwrap());
    } else {
        println!("expected sebs:// and a url. not... whatever that is");
        return;
    }
}
