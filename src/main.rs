use rusqlite::{Connection, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Commune {
    #[serde(rename(deserialize = "codeCommune"))]
    code: String,
    #[serde(rename(deserialize = "nomCommune"))]
    name: String,
    #[serde(rename(deserialize = "libelleAcheminement"))]
    routing_label: String,
    #[serde(rename(deserialize = "codePostal"))]
    postcode: String,
}

fn create_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commune (
            code TEXT PRIMARY_KEY,
            name TEXT NOT NULL,
            routingLabel TEXT NOT NULL,
            postcode TEXT NOT NULL
        )",
        {}
    )?;
    Ok(())
}

fn seed(conn: &Connection) -> Result<()> {
    let f = std::fs::read_to_string("postcodes.json").expect("Could not open the source file.");
    let mut insert_commune = conn.prepare("INSERT INTO commune (name, code, routingLabel, postcode) VALUES (?1, ?2, ?3, ?4)")?;
    let communes: Vec<Commune> = serde_json::from_str(&f).expect("Could not read values inside the source file properly.");
    for commune in &communes {
        insert_commune.execute(
            (
                &commune.name,
                &commune.code,
                &commune.routing_label,
                &commune.postcode,
            ),
        )?;
    }
    Ok(())
}

fn main() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    let db_path = "./postcodes.sqlite";
    let conn = Connection::open(db_path)?;

    let _ = create_db(&conn);
    seed(&conn)
}
