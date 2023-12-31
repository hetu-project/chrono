use accumulator::Configuration;
use clap::Parser;

/// Command line parser
#[derive(Parser)]
struct Cli {
    config_path: String,
    index: usize,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config = Configuration::from_file(&args.config_path);
    let mut server = accumulator::Server::new(&config, args.index).await;
    server.run().await;
}
