/*
 * Copyright (c) 2025 Tim Anhalt (BitTim)
 *
 * Project:    Encountr-CLI
 * License:    GPLv3
 *
 * File:       about.rs
 * Module:     Encountr-CLI
 * Author:     BitTim
 * Modified:   26.10.25, 19:19
 */

use colored::Colorize;

pub(crate) fn about_ui() {
    println!("{}", "Encountr CLI - v0.1.0".bold().bright_green());
    println!("An internal tool to manage the dataset bundled with Encountr");
    println!("{}", "Author: BitTim".dimmed());
}
