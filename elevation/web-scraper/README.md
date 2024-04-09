# Web Scraper

## Introduction

Here is a CLI web scraper that has for only task to download all images it finds in the urls you pass as arguments.  
It looks in the html of the website to find them. Therefore, it does not support cookies and javascript YET.  
(Maybe this will be handle in the future).

## Usage

Either you just want to use the binary on your computer:

```bash
cargo build
mv ./target/debug/web-scraper <PATH OF THE DIRECTORY YOU WANT THE BINARY IN>
./web-scraper [-rlp] <URLS>...
```

Either you can use it from the project folder by just running the following command:

```bash
cargo run -- [-rlp] <URLS>...
```

If you need help:

```bash
cargo run -- --help
```
