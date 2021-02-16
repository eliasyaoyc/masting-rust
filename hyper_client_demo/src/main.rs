use structopt::StructOpt;
use quicli::prelude::*;

const CONN_ADDR: &str = "127.0.0.1:3002";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "url", short = "u")]
    url: String,
    // 配置日志
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    println!("Shortening: {}", args.url);
    let client = reqwest::Client::new();
    let mut res = client
        .post(&format!("http://{}/shorten", CONN_ADDR))
        .body(args.url)
        .send()?;
    let a: String = res.text().unwrap();
    println!("http://{}", a);
    Ok(())
}
