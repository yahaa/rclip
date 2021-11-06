use std::{fs, io::Read, process::exit};

use clap::{App, AppSettings, Arg};
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let matches = App::new("rclip")
        .version("v0.1.0")
        .author("yahaa, yuanzihua0@gmail.com")
        .about("A cross-platform clipboard tool from command line")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![
            App::new("copy")
                .about("Copy a specify file contents to system clipboard")
                .arg(
                    Arg::with_name("src")
                        .help("Where the contents come from, empty represent stdin")
                        .required(false)
                        .index(1),
                ),
            App::new("paste")
                .about("Paste the system clipboard contents to as specify file")
                .arg(
                    Arg::with_name("dst")
                        .help("Where the contents should paste to, empty represent stout")
                        .required(false)
                        .index(1),
                ),
        ])
        .get_matches();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    if let Some(matches) = matches.subcommand_matches("copy") {
        let mut content = String::new();
        if let Some(src) = matches.value_of("src") {
            content = match fs::read_to_string(src) {
                Ok(content) => content,
                Err(e) => {
                    println!("read file {} err {}", src, e);
                    exit(1);
                }
            };
        } else {
            if std::io::stdin().read_to_string(&mut content).is_err() {
                println!("read stdin error");
                exit(1);
            }
        }

        if ctx.set_contents(content).is_err() {
            println!("set clipboard contents error");
            exit(1);
        }
    }

    if let Some(matches) = matches.subcommand_matches("paste") {
        let content = match ctx.get_contents() {
            Ok(content) => content,
            Err(e) => {
                println!("get clipboard contents error {}", e);
                exit(1);
            }
        };

        if let Some(dst) = matches.value_of("dst") {
            if fs::write(dst, content).is_err() {
                println!("write file {} error", dst);
                exit(1);
            }
        } else {
            println!("{}", content);
        }
    }
}
