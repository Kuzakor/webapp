use sled::*;

pub enum DatabaseType {
    Users,
    Games
}

pub fn connect_to_database(which: DatabaseType) -> Db {
    match which {
        DatabaseType::Users => sled::open("databases/users").unwrap(),
        DatabaseType::Games => sled::open("databases/pending_games").unwrap()
    }
}

pub fn get_data_form_database(key: &dyn AsRef<[u8]>, database_type: DatabaseType) -> Option<IVec> {
    let connection = connect_to_database(database_type);
    let value = connection.get(key);
    value.unwrap_or_else(|error| {
        println!("Error at get_data_from_database, reason: {}", error);
        None
    })
}

pub fn get_string_from_binary(binary: IVec) -> String {
    let str = std::str::from_utf8(&binary).unwrap();
    String::from(str)
}
