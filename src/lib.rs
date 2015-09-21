use std::io::BufReader;
use std::io::prelude::*;
use std::process::Child;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub enum Line {
  StdOut(String),
  StdErr(String)
}

pub struct Process<'a> {
  child: &'a mut Child,
  tx: Sender<Option<Line>>,
  rx: Receiver<Option<Line>>,
}

pub struct Lines<'a> {
  process: &'a mut Process<'a>
}

impl <'a> Iterator for Lines<'a> {
  type Item = Line;
  fn next(&mut self) -> Option<Line> {
    match self.process.rx.try_recv() {
      Ok(line) => line,
          _  => None
    }
  }
}

impl<'a> Process<'a> {
  pub fn new(child: &'a mut Child) -> Process<'a> {
    let (tx, rx) = channel();
    Process {
      child: child,
      tx: tx,
      rx: rx,
    }
  }

  pub fn read(&mut self) {
    fn collect<T: Read + Send + 'static>(
      readable: Option<T>, tx: Sender<Option<Line>>
    ) {
      if let Some(r) = readable {
        thread::spawn(move || {
          let reader = BufReader::new(r);
          for line in reader.lines() {
            let _ = match line {
              Ok(l) => tx.send(Some(Line::StdOut(l))),
              _ => tx.send(None)
            };
          }
        });
      } else {
        let _ = tx.send(None);
      }
    };
    collect(self.child.stdout.take(), self.tx.clone());
    collect(self.child.stderr.take(), self.tx.clone());
  }

  pub fn lines(&'a mut self) -> Lines<'a> {
    Lines {
      process: self
    }
  }
}

#[test]
fn it_works() {
}
