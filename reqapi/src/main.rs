use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "An example CLI program.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Hello,
    Reqw,
}

fn say_hello() {
    println!("Hello World!");
}

async fn handle_response(response: reqwest::Response) -> Result<(), reqwest::Error> {
    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            println!("Success! Body: \n{}", body);
        }
        400..=599 => {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Client Error: {} - {}", status, error_message);
        }
        _ => {
            let status = response.status();
            println!("Unexpected status code: {}", status);
        }
    }

    Ok(())
}

async fn get_and_post() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let get_url = "https://httpbin.org/get?foo=bar";
    let get_404_url = "https://httpbin.org/status/404";
    let post_url = "https://httpbin.org/post";
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    // GET
    let get_response = client.get(get_url).send().await?;
    handle_response(get_response).await?;

    let get_404 = client.get(get_404_url).send().await?;
    handle_response(get_404).await?;

    // POST
    let post_response = client
        .post(post_url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;
    handle_response(post_response).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hello => {
            say_hello();
        }
        Commands::Reqw => {
            get_and_post().await?;
        }
    }

    Ok(())
}
