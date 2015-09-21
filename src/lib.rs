use std::io::BufReader;
use std::io::prelude::*;
use std::process::Child;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub enum Line {
  StdOut(String),
  StdErr(String)
}

pub struct Lines {
  rx: Receiver<Option<Line>>,
}

impl Iterator for Lines {
  type Item = Line;
  fn next(&mut self) -> Option<Line> {
    match self.rx.try_recv() {
      Ok(line) => line,
      _  => None,
    }
  }
}

pub fn lines(child: &mut Child) -> Lines {
  let (tx, rx) = channel();
  fn read<R, F>(
    readable: Option<R>,
    tx: Sender<Option<Line>>,
    wrap: F
  ) where
    R: Send + 'static + Read,
    F: Send + 'static + Fn(String) -> Line {
    if let Some(r) = readable {
      thread::spawn(move || {
        let mut buf = BufReader::with_capacity(64, r);
        loop {
          let mut line = String::new();
          match buf.read_line(&mut line) {
            Ok(0) | Err(_)  => {
              print!(""); // not sure why but this is needed for a final *flush*
              let _ = tx.send(None);
              break
            },
            Ok(_)  => {
              let _ = tx.send(Some(wrap(line)));
            }
          }
        }
      });
    } else {
      let _ = tx.send(None);
    }
  };
  read(child.stdout.take(), tx.clone(), |l| Line::StdOut(l));
  read(child.stderr.take(), tx.clone(), |l| Line::StdErr(l));
  Lines { rx: rx }
}

#[test]
fn it_works() {
}
