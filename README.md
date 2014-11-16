rust-madlibs
============

MadLibs for Rust: A game where readers substitute key words into a story template.

Running MadLibs:
Run with ``cargo run``.

Format for story:

Make each word identifier UNIQUE, unless you want to repeat the same word:

``Hello, my name is [Name(1)]. I received the name [Name(1)] after my father's friend, [Name(2)], saved my father's life.``
Here, the command line will prompt for Name(1) and Name(2) once each, filling the input for Name(1) into both slots.
