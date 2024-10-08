use axum::{
    extract::{Query, State},
    response::{ErrorResponse, Json},
    routing::get,
    Router,
};
use clap::Parser;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppConfig {
    db_path: String,
}

#[derive(Debug, Serialize)]
struct Commune {
    #[serde(rename = "codeCommune")]
    code: String,
    #[serde(rename = "nomCommune")]
    name: String,
    #[serde(rename = "libelleAcheminement")]
    routing_label: String,
    #[serde(rename = "codePostal")]
    postcode: String,
}

#[derive(Deserialize)]
struct CommuneFilters {
    #[serde(rename = "codeCommune")]
    code_commune: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input to use to populate the database
    #[arg(short, long, default_value = "db.sqlite")]
    input: String,
}

async fn get_communes(
    State(config): State<AppConfig>,
    Query(filters): Query<CommuneFilters>,
) -> Result<Json<Vec<Commune>>, ErrorResponse> {
    // TODO: handle connection error by impl into StatusCode
    let conn = Connection::open(config.db_path).unwrap();

    let mut query: Vec<String> = Vec::new();
    query.push("SELECT code, name, routingLabel, postcode FROM commune".to_owned());
    if let Some(code) = filters.code_commune {
        if query.len() == 1 {
            query.push("WHERE".to_owned())
        }
        query.push("code =".to_owned());
        query.push(code);
    }

    let mut stmt = conn.prepare(&query.join(" ")).unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut items: Vec<Commune> = Vec::with_capacity(107073);
    while let Some(row) = rows.next().unwrap() {
        items.push(Commune {
            code: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            routing_label: row.get(2).unwrap(),
            postcode: row.get(3).unwrap(),
        });
    }
    Ok(Json(items))
}

#[tokio::main]
async fn run(config: AppConfig) {
    // build our application with a single route

    let api_routes = Router::new()
        .route("/ping", get(|| async { "Hello, World!" }))
        .route("/communes", get(get_communes))
        .with_state(config.clone());
    let app = Router::new().nest("/api/v1", api_routes);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn main() -> Result<()> {
    let args = Args::parse();

    run(AppConfig {
        db_path: args.input,
    });

    Ok(())
}
