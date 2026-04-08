mod cli;
mod tui;
mod clip;
mod ffmpeg;
mod session;

use clip::Clip;
use std::path::PathBuf;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let result = Clip::new(10,30,None);
    let clip: Clip;
    match result {
        Ok(c) => {
            print("Clip created: "); prints(&c.start.to_string()); print(" to "); prints(&c.end.to_string());println("");
            clip = c;
        }
        Err(e) => {
            print("Error: "); prints(&e);println("");
            return;
        }
    };

    let mut source : PathBuf = PathBuf::new();
    source.push("/");
    source.push("home/peter");
    let mut output : PathBuf = PathBuf::new();
    output.push("/");
    output.push("home/peter");


    let a = if 1 == 2 { 1 } else { 2};

    let ffmpeg = ffmpeg::extract_clip(&clip,&source,&output);
    ls2(&["/home","-a"]);
}

fn print(string: &str) {
    prints(&string.to_string());
}

fn println(string: &str) {
    print(&string);
    print("\n");
}

fn prints(string: &String) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(string.as_bytes()).unwrap();
    handle.flush().unwrap();
}

fn printlns(string: &String) {
    prints(string);
    print("\n");
}

pub fn ls(args: &[&str]) {
    let output = Command::new("ls")
        .args(args)
        .output();

    match output {
        Ok(ok) => {
            let okvalue = String::from_utf8(ok.stdout);
            match okvalue {
                Ok(okvalueok) => {
                    printlns(&okvalueok);
                }
                Err(err) => {
                    printlns(&err.to_string());
                }
            }
        }
        Err(err) => {
            printlns(&err.to_string());
        }
    }
}

pub fn ls2(args: &[&str]) -> Result<String, String> {
    let output = Command::new("ls")
        .args(args)
        .output()
        .unwrap();
    return Ok(String::from_utf8(output.stdout).unwrap());
}
