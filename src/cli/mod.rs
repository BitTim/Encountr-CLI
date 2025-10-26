/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Encountr-CLI
 * License:    GPLv3
 *
 * File:       mod.rs
 * Module:     Encountr-CLI
 * Author:     BitTim
 * Modified:   26.10.25, 19:19
 */

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    About,
}
