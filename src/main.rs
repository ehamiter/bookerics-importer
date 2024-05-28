use rusqlite::{params, Connection};
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the bookmarks HTML file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// The path to the output SQLite database file
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

#[allow(dead_code)] // Description and tags are optional; populate in the future
#[derive(Debug)]
struct Bookmark {
    title: String,
    description: String,
    url: String,
    tags: String,
}

fn main() {
    let args = Cli::from_args();

    // Read the HTML file
    let mut file = File::open(args.input).expect("Failed to open input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read input file");

    // Parse the bookmarks
    let bookmarks = parse_bookmarks(&contents);

    // Create the SQLite database
    create_database(args.output, bookmarks);
}

fn parse_bookmarks(contents: &str) -> Vec<Bookmark> {
    let document = Html::parse_document(contents);
    let selector = Selector::parse("a").unwrap();
    let mut bookmarks = Vec::new();

    for element in document.select(&selector) {
        if let Some(url) = element.value().attr("href") {
            let title = element.text().collect::<Vec<_>>().concat();
            bookmarks.push(Bookmark {
                title: title.trim().to_string(),
                description: "".to_string(), // Initialize with empty string
                url: url.to_string(),
                tags: "".to_string(), // Initialize with empty string
            });
        }
    }

    bookmarks
}

fn create_database(path: std::path::PathBuf, bookmarks: Vec<Bookmark>) {
    let mut conn = Connection::open(path).expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            url TEXT NOT NULL,
            tags TEXT
        )",
        [],
    )
    .expect("Failed to create table");

    let tx = conn.transaction().expect("Failed to start transaction");
    {
        let mut stmt = tx
            .prepare("INSERT INTO bookmarks (title, description, url, tags) VALUES (?1, ?2, ?3, ?4)")
            .expect("Failed to prepare statement");

        for bookmark in bookmarks {
            stmt.execute(params![bookmark.title, bookmark.description, bookmark.url, bookmark.tags])
                .expect("Failed to insert bookmark");
        }
    }
    tx.commit().expect("Failed to commit transaction");
}
