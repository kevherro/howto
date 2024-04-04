// MIT License
//
// Copyright (c) 2024 Kevin Herro
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use std::env;

const URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

async fn send_request(
    payload: serde_json::Value,
    api_key: &str,
) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let res = client
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    res.text().await
}

fn main() -> Result<(), Error> {
    let rt =
        tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

    rt.block_on(async {
        let args: Vec<String> = env::args().skip(1).collect();
        let query = args.join(" ");
        let api_key = env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY not found in environment");

        let payload = prepare_payload(&query);
        let response = send_request(payload, &api_key)
            .await
            .expect("Failed to send request");

        if let Ok(api_response) = serde_json::from_str::<ApiResponse>(&response)
        {
            print_choice_message(api_response)
        } else {
            println!("Failed to parse the JSON response");
        }
    });

    Ok(())
}

fn prepare_payload(query: &str) -> serde_json::Value {
    let system_content = format!(
        "Generate a command based on the user's request, ensuring compatibility specifically with {}. Include only the command itself in your response, without any additional explanations or context. Do not format it in any way.",
        env::consts::OS
    );

    let user_content = format!("What is the command to {}?", query);

    json!({
        "model": "gpt-4-turbo-preview",
        "messages": [
            { "role": "system", "content": system_content },
            { "role": "user", "content": user_content }
        ]
    })
}

fn print_choice_message(api_response: ApiResponse) {
    if let Some(first_choice) = api_response.choices.first() {
        println!("{}", first_choice.message.content)
    } else {
        println!("No choices found in the response")
    }
}
