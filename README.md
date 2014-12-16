rust-madlibs
============

MadLibs for Rust: A game where readers substitute key words into a story template.

Running MadLibs:
Run with ``cargo run``.

Format for story:

Make each word identifier UNIQUE, unless you want to repeat the same word:

``Hello, my name is [Name(1)]. My father named me [Name(1)] after the [Noun(1)] that saved his life.``
Here, the command line will prompt for Name(1) and Noun(1) once each, but fill in Name(1) twice.
