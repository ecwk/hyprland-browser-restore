use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, prelude::*};
use std::process::{Command, Stdio};

use hyprland::data::Clients;
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
                    "windowrule monitor {}, title:\"{}\"\n",
                    client.monitor, client.title
                )
                .as_bytes(),
            )
            .unwrap();
        session_file
            .write(
                format!(
                    "windowrule workspace {}, title:\"{}\"\n",
                    client.workspace.id, client.title
                )
                .as_bytes(),
            )
            .unwrap();
    }
}

pub fn load_session(path: &str) {
    let session_file = OpenOptions::new().read(true).open(path).unwrap();
    let reader = BufReader::new(session_file);

    for line in reader.lines() {
        let line = line.unwrap();

        let hyprctl = Command::new("hyprctl")
            // `hyprctl source` works too, but, it causes an expected behaviour where certain modifier key holds are reset. For
            // example, holding `ctrl-u` to scroll will cause `u `to be pressed instead when `hyprctl
            // source` is ran.
            .args(["keyword", &line])
            .stdout(Stdio::null())
            .status()
            .unwrap();

        assert_eq!(hyprctl.success(), true);
    }
}
