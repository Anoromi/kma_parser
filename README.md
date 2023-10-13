
# Kma parser
The program parses xslx files for different specialties.
The program *seems* to work for both economists and SE.

## How to run

1. Insall Rust
2. Run `cargo run -- parse -h` and follow cli guidelines from there.
3. A call might look something like this `cargo run -- parse -c configs/softies.json --file spreadsheet/3.xlsx -o ./hell.json`

### Aspects
Some changes were made to the task:
  1. Resulting json file skips Faculty object.
  1. Group might have multiple objects inside of it (2 different lectures, for example). As such a group was changed to an array of objects instead of a single object.

### Structure
  1. The program doesn't do any seaching inside a spreadsheet. It starts at 0 row, 0 col and goes from there.
  1. I did some testing but not enough.
  1. The program uses configs for specifying what kinds of specialties it is currently working on. This is used both for aliases and default specialties.

## Rust
Rust was chosen for this project because:
  1. I like the language. (I use rust btw).
  2. It's a good choice for a cli application and I figgured the program would be one.
  3. It's a good language when you know the structure of a task beforehand.

Some of the libraries used were:
  1. calamine - a library for reading xslx files.
  2. nom - a parsing library used for parsing various strings.
  3. anyhow - a library for unified errors.
  4. serde - a framework for serialization.
  5. clap - a framework for building a cli app.



