use anyhow::Result;
use clap::Parser;
use cmd::{build::Build, create::Create, publish::Publish};

pub mod cmd;

#[derive(Parser)]
#[clap(author, version)]
pub struct Arguments {
    #[clap(long, short, global = true)]
    pub package: Option<String>,
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    Create(Create),
    Build(Build),
    Publish(Publish),
}

pub async fn run_cli() -> Result<()> {
    let args = Arguments::parse();

    match args.cmd {
        Command::Create(r) => r.execute(),
        Command::Build(r) => r.execute(args.package).await,
        Command::Publish(r) => r.execute(args.package).await,
    }
}
