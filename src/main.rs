use rusqlite::{Connection, OptionalExtension, Result};
use std::fs;
use std::process::Command;

static DB_PATH: &str = "database.db";
static MIGRATION_PATH: &str = "migration.sql";
static TEST_SQL_PATH: &str = "test.sql";

fn main() {
    administer_challenge().unwrap();
}

fn administer_challenge() -> Result<()> {
    create_db_and_bail_if_missing()?;

    let conn = Connection::open(DB_PATH)?;

    check_if_user_attempted_challenge(&conn)?;
    evaluate_users_solution(&conn)
}

fn create_db_and_bail_if_missing() -> Result<()> {
    if std::path::Path::new(DB_PATH).exists() {
        return Ok(());
    }

    println!("🧱 {DB_PATH} not found — constructing the Cubical Dungeon...");

    if !std::path::Path::new(MIGRATION_PATH).exists() {
        eprintln!("❌ {MIGRATION_PATH} missing — cannot build {DB_PATH}");
        std::process::exit(1);
    }

    let status = Command::new("sqlite3")
        .arg(DB_PATH)
        .arg(format!(".read {}", MIGRATION_PATH))
        .status()
        .expect("failed to run sqlite3");

    if !status.success() {
        eprintln!("❌ sqlite3 failed to apply migration");
        std::process::exit(1);
    }

    println!("✅ Dungeon constructed! database.db is ready.");
    println!("To explore the dungeon manually:");
    println!("  sqlite3 {DB_PATH}");
    println!("Inside SQLite, view the schema with:");
    println!("  .schema");

    get_ret("Exiting")
}

fn check_if_user_attempted_challenge(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='view' AND name='strongest_monsters';",
    )?;

    if let Ok(attempt) = stmt
        .query_row([], |row| row.get::<usize, String>(0))
        .optional()
    {
        match attempt {
            None => {
                println!("🧙 The Warden whispers: You have not yet attempted the challenge.");
                println!("The dungeon is ready. Create your solution as:");
                println!("  CREATE VIEW strongest_monsters AS ... ;");
                println!("");
                println!("To inspect the database schema:");
                println!("  sqlite3 {DB_PATH}");
                println!("  .schema");

                get_ret("Exiting")
            }
            Some(_) => Ok(()),
        }
    } else {
        println!("Something is bad with querying the database?");
        get_ret("Exiting")
    }
}

fn evaluate_users_solution(conn: &Connection) -> Result<()> {
    println!("🔍 Attempt detected! Evaluating your solution...");

    let test_sql = fs::read_to_string(TEST_SQL_PATH).expect("Could not read test.sql");

    let mut stmt = conn.prepare(&test_sql)?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?, // cube_id
            row.get::<_, i64>(1)?, // monster_id
            row.get::<_, i64>(2)?, // is_correct
        ))
    })?;

    let mut all_correct = true;
    println!("\n📊 Test Results:");
    for row in rows {
        let (cube_id, monster_id, is_correct) = row?;
        if is_correct == 1 {
            println!(" cube {} → monster {} ✔ correct", cube_id, monster_id);
        } else {
            println!(" cube {} → monster {} ✘ incorrect", cube_id, monster_id);
            all_correct = false;
        }
    }

    println!();
    if all_correct {
        println!("🏆 **You have mastered the Cubical Dungeon’s first trial!**");
    } else {
        println!("❌ Some answers were incorrect.");
        println!("The Warden mutters: 'Refine your query, wanderer.'");
    }

    Ok(())
}

fn get_ret(msg: &str) -> std::result::Result<(), rusqlite::Error> {
    Err(rusqlite::Error::SqliteFailure(
        rusqlite::ffi::Error::new(0),
        Some(msg.into()),
    ))
}
