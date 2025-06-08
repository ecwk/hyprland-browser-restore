use std::{env, fs, thread, time};

use clap::Parser;
use hyprland::shared::*;

mod session;
use session::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<String>,
}

fn main() -> hyprland::Result<()> {
    let args = Args::parse();
    let path = args
        .path
        .unwrap_or(env::var("XDG_STATE_HOME").unwrap() + "/hyprland-chrome-restore");
    if fs::exists(&path).unwrap() {
        load_session(&path);
    }

    loop {
        if is_browser_running("chrome") {
            create_session(&path);
            save_session(&path);
            load_session(&path);
        }

        thread::sleep(time::Duration::from_secs(1))
    }
}
