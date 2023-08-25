use crate::database;
use core::panic;
use uuid::Uuid;
//use bitcode::{encode, decode};
use sled::*;

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub uuid: String,
}

pub fn register_new_user(username: String, email: String, password: String) -> User {
    let uuid = Uuid::new_v4().to_string();
    let user = User {
        username,
        email,
        password,
        uuid,
    };
    let serialized_user = bitcode::encode(&user).unwrap();
    let database_conection = database::connect_to_database(database::DatabaseType::Users);

    let _ = database_conection.insert(&user.username, user.uuid.as_str());
    let _ = database_conection.insert(&user.uuid, serialized_user);
    user
}

pub fn get_user_uuid_by_username(username: String) -> Option<String> {
    let uuid_binary = database::get_data_form_database(&username, database::DatabaseType::Users);
    uuid_binary.map(database::get_string_from_binary)
}

pub fn get_user_from_databse(uuid: String) -> User {
    let user_in_binary = database::get_data_form_database(&uuid, database::DatabaseType::Users);
    get_user_from_binary(user_in_binary)
}

pub fn get_user_from_binary(binary_data: Option<IVec>) -> User {
    match binary_data {
        None => panic!("Error converting data back to User type: no data"),
        Some(data) => bitcode::decode(&data).unwrap(),
    }
}

