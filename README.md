
# Kma parser
The program parses xslx files for different specialties in university.
The program works for both economists and software engineers.

## How to run

1. Insall Rust
2. Run `cargo run -- parse -h` and follow cli guidelines from there.
3. Specify a config for your specialty. They are located in configs directory. (configs/econ.json config/softies.json)
4. A call might look something like this `cargo run -- parse -c configs/softies.json --file spreadsheet/3.xlsx -o ./hell.json`. The output will be written to hell.json file.

### Modifications
Some changes were made to the task:
  1. Resulting json file skips Faculty object.
  1. Group might have multiple objects inside of it (2 different lectures, for example). As such a group was changed to an array of objects instead of a single object.

### Structure
  1. The program doesn't do any seaching inside a spreadsheet. It starts at 0 row, 0 col and goes from there.
  1. I did some testing but not enough. To run test use command `cargo test`
  1. The program uses configs for specifying what kinds of specialties it is currently working on. Default specialty is used when no specialty is found inside a course title.
  1. Aliases are matched with or without a period on the end. A lowercase substring is considered to be enough to match alias with a name. For example 'марк' is a substring of 'Маректинг'. The caveat is that an alias can be mached to more than one specialty.

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
  5. clap - a framework for building cli apps.
