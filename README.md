# bookerics-importer

blazing fast [bookerics](https://github.com/ehamiter/bookerics) importer

## what?

This takes in a `<!DOCTYPE NETSCAPE-Bookmark-file-1>`-compliant 
generated file from a browser or web service like pinboard.in and transmogrifies it into a sqlite3 db file, suitable for use in a web application like bookerics™.

## buildage

```
❯ cargo build --release
```

## usage

```
❯ bookerics_importer --help
bookerics_importer 0.1.0

USAGE:
    bookerics_importer <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>     The path to the bookmarks HTML file
    <output>    The path to the output SQLite database file
```
