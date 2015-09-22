use std::io::BufReader;
use std::io::prelude::*;
use std::process::Child;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

/// represents a single line of output from a child process
#[derive(Debug)]
pub enum Line {
  /// a line of stdout output
  StdOut(String),
  /// a line of stderr output
  StdErr(String)
}

/// represents a source of child process line output
pub struct Lines {
  rx: Receiver<Option<Line>>,
}

/// represents an iterable of Line instances
pub struct Iter<'a> {
  lines: &'a Lines
}

impl Lines {
  pub fn iter(&self) -> Iter {
    Iter { lines: self }
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item = Line;
  fn next(&mut self) -> Option<Line> {
    match self.lines.rx.recv() {
      Ok(line) => line,
            _  => None,
    }
  }
}

/// creates a new Lines instance
///
/// ```rust
///  match Command::new("...").spawn() {
///    Ok(mut child) => {
///       let lines = pines::lines(&mut child);
///       child.wait().unwrap();
///       for l in lines.iter() {
///          println!("{}", l)
///       }
///    }
///    _ => println!("failed to launch process")
///  }
/// ```
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
              print!("\n"); // not sure why but this is needed for a final *flush*
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
