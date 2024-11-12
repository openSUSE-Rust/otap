// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela
// Copyright Gordon Leung

use clap::Parser;
use otap::cli::{Cli, Command, RequestCommand};
use otap::credentials::{ask_credentials, Credentials};

#[tokio::main]
async fn main() {
    let Cli { verbose: _, quiet: _, command } = Cli::parse();

    let result = match command {
        Command::Request(cmd) => match cmd {
            RequestCommand::Show { id, .. } => match ask_credentials() {
                Ok(Credentials { username, password }) => {
                    libotap::requests::get(id, username, password).await
                }
                Err(err) => Err(err.to_string()),
            },
        },
    };

    match result {
        Ok(_) => {
            tracing::info!("Completed");
        }
        Err(error) => {
            tracing::error!(error = ?error);
            eprintln!("{}", error);
            std::process::exit(1)
        }
    }
}
