use project::{create_table, delete_item, delete_table, get_items, insert_item, update_item};

#[test]
fn test_delete_table() {
    assert_eq!(delete_table().is_ok(), true);
}

#[test]
fn test_create_table() {
    assert_eq!(create_table().is_ok(), true);
}

#[test]
fn test_insert_item() {
    delete_table().unwrap();
    create_table().unwrap();
    assert_eq!(
        insert_item("John", "he is a graduate student").is_ok(),
        true
    );
    assert_eq!(insert_item("Mary", "she is a teacher").is_ok(), true);
}

#[test]
fn test_get_items() {
    delete_table().unwrap();
    create_table().unwrap();
    insert_item("John", "he is a graduate student").unwrap();
    assert_eq!(get_items().is_ok(), true);
}

#[test]
fn test_update_item() {
    delete_table().unwrap();
    create_table().unwrap();
    insert_item("John", "he is a graduate student").unwrap();
    assert_eq!(
        update_item(1, "John", "he is a graduate student in Computer Science").is_ok(),
        true
    );
}

#[test]
fn test_delete_item() {
    delete_table().unwrap();
    create_table().unwrap();
    insert_item("John", "he is a graduate student").unwrap();
    assert_eq!(delete_item(1).is_ok(), true);
}
