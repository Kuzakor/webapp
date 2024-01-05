use crate::database;
use core::panic;
use uuid::Uuid;
//use bitcode::{encode, decode};

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub uuid: String,
}

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct Game {
    player_one: User,
    player_two: User,
    uuid: String,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
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

    pub fn get_uuid_by_username(username: String) -> Option<String> {
        let uuid_binary = database::get_data_form_database(&username, database::DatabaseType::Users);
        uuid_binary.map(database::get_string_from_binary)
    }

    pub fn get_from_databse(uuid: String) -> Self {
        let user_in_binary = database::get_data_form_database(&uuid, database::DatabaseType::Users);
        match user_in_binary {
            None => panic!("Error converting data back to desired type: no data"),
            Some(data) => bitcode::decode(&data).unwrap(),
        }
    }

}

impl Game {
    pub fn new(player_one: User, player_two: User) -> Self {
        let uuid = Uuid::new_v4().to_string();
        let game = Game {
            player_one,
            player_two,
            uuid,
        };
        let serialized_game = bitcode::encode(&game).unwrap();
        let database_conection = database::connect_to_database(database::DatabaseType::Games);

        let _ = database_conection.insert(&game.uuid, serialized_game);
        game
    }

    pub fn get_from_databse(uuid: String) -> Self {
        let user_in_binary = database::get_data_form_database(&uuid, database::DatabaseType::Games);
        match user_in_binary {
            None => panic!("Error converting data back to desired type: no data"),
            Some(data) => bitcode::decode(&data).unwrap(),
        }
    }
}



