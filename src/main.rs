use rusqlite::{params, Connection};
use scraper::{Html, Selector, ElementRef};
use std::fs::File;
use std::io::Read;
use structopt::StructOpt;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(StructOpt)]
struct Cli {
    /// The path to the bookmarks HTML file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    /// The path to the output SQLite database file
    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct Bookmark {
    title: String,
    description: String,
    url: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
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
    let dt_selector = Selector::parse("dt").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    // let dd_selector = Selector::parse("dd").unwrap();

    let mut bookmarks = Vec::new();

    for dt_element in document.select(&dt_selector) {
        if let Some(link_element) = dt_element.select(&a_selector).next() {
            if let Some(url) = link_element.value().attr("href") {
                let title = link_element.text().collect::<Vec<_>>().concat();
                let mut description = String::new();

                if let Some(next_sibling) = dt_element.next_sibling() {
                    if let Some(next_dd) = ElementRef::wrap(next_sibling) {
                        if next_dd.value().name() == "dd" {
                            description = next_dd.text().collect::<Vec<_>>().concat();
                        }
                    }
                }

                let tags = link_element
                    .value()
                    .attr("tags")
                    .unwrap_or("")
                    .split(',')
                    .map(String::from)
                    .collect::<Vec<_>>();
                let add_date = link_element.value().attr("add_date").unwrap_or("0");
                let timestamp = add_date.parse::<i64>().unwrap();
                let created_at = Utc.timestamp_opt(timestamp, 0).single().expect("Invalid timestamp");
                let updated_at = Utc::now();

                bookmarks.push(Bookmark {
                    title: title.trim().to_string(),
                    description: description.trim().to_string(),
                    url: url.to_string(),
                    tags,
                    created_at,
                    updated_at,
                });
            }
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
            tags TEXT,
            created_at TEXT,
            updated_at TEXT
        )",
        [],
    )
    .expect("Failed to create table");

    let tx = conn.transaction().expect("Failed to start transaction");
    {
        let mut stmt = tx
            .prepare("INSERT INTO bookmarks (title, description, url, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)")
            .expect("Failed to prepare statement");

        for bookmark in bookmarks {
            let tags = serde_json::to_string(&bookmark.tags).expect("Failed to serialize tags");
            stmt.execute(params![
                bookmark.title, 
                bookmark.description, 
                bookmark.url, 
                tags, 
                bookmark.created_at.to_rfc3339(), 
                bookmark.updated_at.to_rfc3339()
            ])
            .expect("Failed to insert bookmark");
        }
    }
    tx.commit().expect("Failed to commit transaction");
}
