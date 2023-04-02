# Grep Clone
Implementation of https://doc.rust-lang.org/book/ch12-00-an-io-project.html

This is a toy project that works by supporting searching for a string in a file.

## Installation
```bash
cargo install grep-clone
```

## Usage
```sh
grep-clone string file.txt
```

It also supports ignore-case
```sh
grep-clone string file.txt -i
```

Or
```sh
grep-clone string file.txt --ignore-case
```

Or
```
export IGNORE_CASE=true
grep-clone string file.txt
```
