extern crate pine;
use std::process::{Command, Stdio};

fn main() {
    let mut process = Command::new("/bin/sh")
        .arg("-c")
        .arg("curl https://www.howsmyssl.com/a/check")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().ok().unwrap();
    let lines = pine::lines(&mut process);
    for line in lines.iter() {
        match line {
            pine::Line::StdOut(line) => println!("out -> {}", line),
            pine::Line::StdErr(line) => println!("err -> {}", line)
        }
    }
}
