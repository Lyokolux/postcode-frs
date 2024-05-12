use rusqlite::Connection;
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

pub fn create_db(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commune (
                code TEXT PRIMARY_KEY,
                name TEXT NOT NULL,
                routingLabel TEXT NOT NULL,
                postcode TEXT NOT NULL
            )",
        {},
    )
}

pub fn seed(conn: &Connection, input: &str) -> Result<(), rusqlite::Error> {
    let f = std::fs::read_to_string(input).expect("Could not open the source file.");
    let insert_commune_prep = conn.prepare(
        "INSERT INTO commune (name, code, routingLabel, postcode) VALUES (?1, ?2, ?3, ?4)",
    );
    if insert_commune_prep.is_err() {
        return Err(insert_commune_prep.unwrap_err());
    }

    let mut insert_commune = insert_commune_prep.unwrap();

    let communes: Vec<Commune> =
        serde_json::from_str(&f).expect("Could not read values inside the source file properly.");
    for commune in &communes {
        let _ = insert_commune.execute((
            &commune.name,
            &commune.code,
            &commune.routing_label,
            &commune.postcode,
        ));
    }
    Ok(())
}
