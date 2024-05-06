mod api;

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Subcommand)]
enum Action {
    Show,

    Track {
        #[arg(short, long, value_name = "TIME")]
        time: Option<String>,
    },

    Update {
        #[arg(short, long, value_name = "TIME")]
        time: Option<String>,

        #[arg(short, long, value_name = "SLOT")]
        slot: Option<String>,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    action: Option<Action>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.action {
        Some(Action::Show) => {
            let day = api::today();

            println!("{:?}", day);
        }
        Some(Action::Track { time }) => {
            let day = api::enter("start1".to_owned(), time.to_owned());

            println!("{:?}", day);
        }
        Some(Action::Update { time, slot }) => {
            println!("update");
        }
        None => {}
    }

    ExitCode::SUCCESS
}
