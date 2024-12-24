use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    pub template_clash_config: String,
    #[arg(long)]
    pub subscribe_url: String,
    #[arg(long)]
    pub data_dir: String,
}

