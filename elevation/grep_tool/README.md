# Simple grep CLI

## Objectives
This small project is the first of a group of ten projects.
Their goal is to improve my Rust coding skills.

As its name implies, it is a small grep tool that finds a pattern in files.
It prints each line that has the pattern alongside the file name and its line number.


## Usage
You might have already clone the meta-directory that holds all the projects.
I assume that you are in the directory /Rust_is_Great/elevation

```bash
cd grep_tool
cargo run -- <PATTERN> <PATH>
```

You can also build the executable and run it from the /target/bin directory.
The tool handles as may files as you like but one pattern.
If your pattern has many words, I suggest that you use quotes to delimitate it.
