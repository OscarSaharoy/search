# search

This is a small tool that lets you search the web from the command line.

Example usage:

```bash
[oscarsaharoy ~] $ search rust docs

Learn Rust - Rust Programming Language [https://www.rust-lang.org/learn]
Documentation. Read the core documentation. All of this documentation is also
available locally using the rustup doc command, which will ...

Rust Documentation [https://doc.rust-lang.org/]
Welcome to an overview of the documentation provided by the Rust project.
This page contains links to various helpful references, most of which are
available ...

... more results ...

Open in ecosia: https://www.ecosia.org/search?q=rust%20docs
```

To get it up and running, I built and linked it into my path using `cargo build -r; ln target/release/search ~/bin/search` and I can just run it like `search anything` :)

I couldn't find anywhere it said this type of scraping was against ecosia's policy so hopefully its ok!!!

