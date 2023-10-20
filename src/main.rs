use src::{create_table, insert_item, get_items, update_item, delete_item, delete_table};
use rusqlite::Error;

fn main() -> Result<(), Error> {
    delete_table()?;
    create_table()?;

    insert_item("John", "he is a graduate student")?;
    insert_item("Mary", "she is a teacher")?;
    
    println!("Persons in the database:");
    get_items()?;
    
    update_item(1, "John", "he is a graduate student in Computer Science")?;
    println!("Persons in the database after update:");
    get_items()?;
    
    delete_item(1)?;
    println!("Persons in the database after delete:");
    get_items()?;
    
    Ok(())
}