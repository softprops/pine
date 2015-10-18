# pine

[![Build Status](https://travis-ci.org/softprops/pine.svg?branch=master)](https://travis-ci.org/softprops/pine)

> line oriented process output

## apidocs

Find them [here](http://softprops.github.io/pine)

## usage

Rust's interface for working with [processes](https://doc.rust-lang.org/std/process/) is pretty great, but sometimes
you may wish to stream process output as it becomes available, rather waiting for the process to exit before you can get
and handle on the total process out [output](https://doc.rust-lang.org/std/process/struct.Output.html).

For these usecases, `pine` provides in iterator interface over lines of process output,
represented as enum of `pine.Line.StdOut` or `pine.Line.StdErr`. This is well suited for unix programs with emit
line-oriented output. A prerequite for your program to gain access
to these lines of output, is making sure your child process output is "piped" to your program. Rust's Command interface
makes this simple.

```rust
extern crate pine;
use std::process::{Command, Stdio};

let mut process = Command::new("/bin/sh")
    .arg("-c")
    .arg("curl https://www.howsmyssl.com/a/check")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn().ok().unwrap();
```

With the child's output piped to your program, you can then iterate over lines of output as
they are available. using the `pine::lines` function.

```rust
use pine::Line;
let lines = pine::lines(&mut process);
for line in lines.iter() {
    match line {
        Line::StdOut(line) => println!("out -> {}", line),
        Line::StdErr(line) => println!("err -> {}", line)
    }
}
```

Note `iter()` returns an iterator, which means any functions defined on iterator are
at your disposal for processing line output.

Doug Tangren (softprops) 2015
