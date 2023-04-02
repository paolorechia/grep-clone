# Mini Grep (clone)
Implementation of https://doc.rust-lang.org/book/ch12-00-an-io-project.html

This is a toy project that works by supporting searching for a string in a file.

Usage:
```
mini-grep string file.txt
```

It also supports ignore-case
```
mini-grep string file.txt -i
```

Or
```
mini-grep string file.txt --ignore-case
```

Or
```
export IGNORE_CASE=true
mini-grep string file.txt
```
