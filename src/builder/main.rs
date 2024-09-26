use clap::Parser;
use rusqlite::{Connection, Result};
use std::path::Path;

mod db;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input to use to populate the database
    #[arg(short, long, default_value = "input.json")]
    input: String,

    /// The name of the sqlite database
    #[arg(short, long, default_value = "db.sqlite")]
    output: Option<String>,
}

fn derive_db_name_from(filepath: &str) -> String {
    format!(
        "{}.{}",
        Path::new(filepath).file_stem().unwrap().to_str().unwrap(),
        "sqlite"
    )
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    let default_output = args
        .output
        .or(Some(derive_db_name_from(&args.input)))
        .unwrap();
    let mut conn = Connection::open(default_output)?;

    db::create_db(&conn)?;
    db::seed(&mut conn, &args.input)?;
    Ok(())
}
