extern crate pines;

use std::process::{Command, Stdio};

fn main() {
  match Command::new("/bin/sh")
    .arg("-c")
    .arg("curl https://www.howsmyssl.com/a/check")
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn() {
      Err(_) => {
        panic!("failed to exec cmd")
      },
      Ok(mut child) => {
        let lines = pines::lines(&mut child);
//        child.wait().unwrap();
        for line in lines.iter() {
          println!("{:?}", line)
        }
      }
    };
}
