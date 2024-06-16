# bookerics-importer

blazing fast [bookerics](https://github.com/ehamiter/bookerics) importer


## what?

This takes in a `<!DOCTYPE NETSCAPE-Bookmark-file-1>`-compliant 
generated file from a browser or web service like pinboard.in and transmogrifies it into a sqlite3 db file, suitable for use in a web application like bookerics™.

## table structure
```
PRAGMA table_info(bookmarks);

[
 (0, 'id', 'INTEGER', 0, None, 1),
 (1, 'title', 'TEXT', 1, None, 0),
 (2, 'description', 'TEXT', 0, None, 0),
 (3, 'url', 'TEXT', 1, None, 0),
 (4, 'thumbnail_url', 'TEXT', 0, None, 0),  # <initially left blank for future population>
 (5, 'tags', 'TEXT', 0, None, 0),
 (6, 'created_at', 'TEXT', 1, "datetime('now')", 0),
 (7, 'updated_at', 'TEXT', 1, "datetime('now')", 0)
]
```


## buildage

```
❯ cargo build --release
```
executable will be built in `./target/relase/`. link it somewhere or move it somewhere on your PATH


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
