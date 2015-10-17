# pine

[![Build Status](https://travis-ci.org/softprops/pine.svg?branch=master)](https://travis-ci.org/softprops/pine)

> rust process line output

## apidocs

Fine them [here](http://softprops.github.io/pine)

## usage

Rust's interface for working with [processes](https://doc.rust-lang.org/std/process/) is great but sometimes
you may wish to stream process output as its available rather waiting for the process to exist before collecting
processes total [output](https://doc.rust-lang.org/std/process/struct.Output.html).

For these usecases, `pine` provides in iterator interface over lines of process output,
represented as enum of `pine.Line.StdOut` or `pine.Line.StdErr`. A prerequite to your program gaining access
to these lines of output, is making sure your child process output is "piped" to your program.

```rust
extern crate pine;
use std::process::{Command, Stdio};

fn main() {
  let mut process = Command::new("/bin/sh")
    .arg("-c")
    .arg("curl https://www.howsmyssl.com/a/check")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn().ok().unwrap();
```

With the subprocess piped to your program you can then iterate over lines of output as
they are available.

```rust
  let lines = pine::lines(&mut process);
  for line in lines.iter() {
    println!("{:?}", line);
  }
}
```

Doug Tangren (softprops) 2015
