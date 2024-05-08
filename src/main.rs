mod api;
mod cli;
mod file;
mod slot;
mod structs;

use clap::Parser;

fn main() -> Result<(), structs::Error> {
    match &cli::Cli::parse().action {
        Some(cli::Action::Show) => api::show(),
        Some(cli::Action::Track { time }) => api::track(time),
        Some(cli::Action::Update { time, slot }) => api::update(time, slot),
        None => Ok(()),
    }
}
