# rumdr
## Markdown parsing in the terminal

This is still very work-in-progress so don't expect anything useful yet!

### Roadmap

I expect parsing to work in this order:

1. Check for multi-line formatting (e.g. [fenced code blocks](https://www.markdownguide.org/extended-syntax/#fenced-code-blocks))
2. Check for single-line formatting (e.g. bullet points)
3. Parse full text on a per-character basis (e.g. italics)

It's possible that this design will change as I get farther along in the project.

### Expected features

* ``less``-like ability to scan through the document with arrow keys and ``vim``-like bindings
* basic configuration via env and a config file for colors and styles
* option to turn headers into cool ascii art using [figlet-rs](https://lib.rs/crates/figlet-rs) or something similar (will be off by default in favor of something boring)

### How to run

Building works the usual way for a Rust project:

1. Make sure you have the latest version of ``cargo``, Rust's package manager.
2. ``git clone`` this repository into your desired location
3. ``cd`` into the ``./rumdr`` folder
4. Use ``cargo run`` to run a development build
5. Use ``cargo build --release`` to make a release build
