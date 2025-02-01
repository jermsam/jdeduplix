// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Initialize logging with pretty formatting
    tracing_subscriber::fmt()
        .with_env_filter("jdeduplix=debug")
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_level(true)
        .with_ansi(true)
        .init();

    jdeduplix::run()
}
