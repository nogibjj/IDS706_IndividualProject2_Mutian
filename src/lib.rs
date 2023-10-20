extern crate rusqlite;

#[allow(deprecated)]
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
// Create
#[allow(deprecated)]
pub fn create_table() -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  description     TEXT
                  )",
        NO_PARAMS,
    )?;
    Ok(())
}
// Insert
pub fn insert_item(name: &str, description: &str) -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "INSERT INTO person (name, description) VALUES (?1, ?2)",
        &[name, description],
    )?;
    Ok(())
}
// Read

pub fn get_items() -> Result<()> {
    let conn = Connection::open("test.db")?;
    let mut stmt = conn.prepare("SELECT id, name, description FROM person")?;

    let items = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, i32>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?,
        ))
    })?;
    for item in items {
        match item {
            Ok(item) => println!(
                "id: {:?}, name: {:?}, description: {:?}",
                item.0, item.1, item.2
            ),
            Err(err) => println!("Error: {:?}", err),
        }
    }
    Ok(())
}
// Update
pub fn update_item(id: i32, name: &str, description: &str) -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute(
        "UPDATE person SET name = ?1, description = ?2 WHERE id = ?3",
        &[name, description, &id.to_string()],
    )?;
    Ok(())
}

// Delete
pub fn delete_item(id: i32) -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute("DELETE FROM person WHERE id = ?1", &[&id])?;
    Ok(())
}

// Delete table

pub fn delete_table() -> Result<()> {
    let conn = Connection::open("test.db")?;
    conn.execute("DROP TABLE IF EXISTS person", NO_PARAMS)?;
    Ok(())
}
