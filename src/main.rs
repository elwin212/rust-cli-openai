use hyper::body::Buf;
use hyper::{Body, header, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};


#[derive(Serialize,Deserialize, Debug)]
struct ChatCompletionRequestMessage{
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct MessageFromOpenAi{
    index: u32,
    message: Vec<ChatCompletionRequestMessage>,
    finish_reason: String,
}

#[derive(Serialize, Debug)]
struct RequestForOpenAi{
    model: String,
    messages: Vec<ChatCompletionRequestMessage>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    n: u32,    
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let url = "https://api.openai.com/v1/chat/completions";

    let system_message = "Your name is Elwin, You are a helpful assistant, answer the queries in a fun way.";

    let api_key:String = env::var("OPENAI_API_KEY").unwrap();

    let auth_header_val = format!("Bearer {}", api_key);

    println!(
        "{esc}c",
        esc = 27 as char
    );
    
    loop {
        print!("Enter the message: >");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Error while reading the message");

        println!("");

        let mut sp = Spinner::new(Spinners::Dots9, "\t\tLet me think for a sec...".into());        

        let payload = RequestForOpenAi{
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![ChatCompletionRequestMessage{
                role: String::from("system"),
                content: system_message.to_string(),
            },
            ChatCompletionRequestMessage {
                role: String::from("user"),
                content: user_input.to_string(),
            },
            ],
            max_tokens: 250,
            temperature: 0.9,
            top_p: 1.0,
            n: 1,            
        };

        let body = Body::from(serde_json::to_string(&payload)?);
        
        let req = Request::post(url)
            .header(header::AUTHORIZATION, &auth_header_val)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap();
        
        let resp = client.request(req).await?;

        let body_resp = hyper::body::aggregate(resp).await?;

        let json : MessageFromOpenAi = serde_json::from_reader(body_resp.reader())?;

        sp.stop();

        println!("");
        
        println!("{}" , json.message[0].content.to_string());
    }

    Ok(())
}

