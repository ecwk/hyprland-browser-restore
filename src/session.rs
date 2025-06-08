use std::fs;
use std::fs::OpenOptions;
use std::io::{prelude::*};
use std::process::{Command, Stdio};

use hyprland::data::{Clients};
use hyprland::prelude::*;

pub fn is_browser_running(process_name: &str) -> bool {
    let ps = Command::new("ps")
        .args(["aux"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep = Command::new("grep")
        .arg(format!("[{}]{}", &process_name[0..1], &process_name[1..]))
        .stdin(Stdio::from(ps.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut output = String::new();
    grep.stdout.unwrap().read_to_string(&mut output).unwrap();

    let is_running = !output.trim().is_empty();
    is_running
}

pub fn create_session(path: &str) {
    fs::File::create(path).unwrap();
}

pub fn save_session(path: &str) {
    let mut session_file = OpenOptions::new().append(true).open(path).unwrap();

    for client in Clients::get().unwrap().iter() {
        let is_browser = client.class.to_lowercase().contains("google")
            && client.class.to_lowercase().contains("chrome");
        if !is_browser {
            continue;
        }

        session_file
            .write(
                format!(
                    "windowrule = monitor {}, title:{}\n",
                    client.monitor, client.title
                )
                .as_bytes(),
            )
            .unwrap();
        session_file
            .write(
                format!(
                    "windowrule = workspace {}, title:{}\n",
                    client.workspace.id, client.title
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

pub fn load_session(path: &str) {
    let hyprctl = Command::new("hyprctl")
        .args(["keyword", "source", path])
        .stdout(Stdio::null())
        .status()
        .unwrap();

    assert_eq!(hyprctl.success(), true);
}
