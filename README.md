rust-madlibs
============

MadLibs for Rust: A game where readers substitute key words into a story template.

Running MadLibs:
Run with ''cargo run''.

Format for story:

Make each word identifier UNIQUE, unless you want to repeat the same word:

He was [adjective-1], and she was also [adjective-1], but she eventually calmed down and seemed [adjective-2]
-> this will only prompt you for two adjectives, and will repeat the first one you input twice.
