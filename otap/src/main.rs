// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela
// Copyright Gordon Leung

use clap::Parser;
use libotap::general_information::service::get_services;

#[derive(Parser, Debug)]
enum Cli {
    /// Show which services are known to OBS
    Services
}

fn main() {
    let result = match Cli::parse() {
        Cli::Services => get_services()
    };

    match result {
        Ok(_) => {
            tracing::info!("Completed")
        }
        Err(error) => {
            tracing::error!(error = ?error);
            eprintln!("{}", error);
            std::process::exit(1)
        }
    }
}
