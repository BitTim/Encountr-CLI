/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Encountr-CLI
 * License:    GPLv3
 *
 * File:       main.rs
 * Module:     Encountr-CLI
 * Author:     BitTim
 * Modified:   26.10.25, 19:19
 */

use crate::cli::{Cli, Commands};
use crate::ui::about::about_ui;
use clap::Parser;

mod cli;
mod db;
mod ui;

fn main() {
    let cli = Cli::parse();
    match_command(&cli.command)
}

fn match_command(cmd: &Commands) {
    match cmd {
        Commands::About => about_ui(),
    }
}
