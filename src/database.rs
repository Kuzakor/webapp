
use sled::*;


pub fn connect_to_database() -> Db
{
    let database: sled::Db = sled::open("/tmp/test").unwrap();
    database
}

pub fn get_data_form_database(key: &dyn AsRef<[u8]>) -> Option<IVec>
{
    let connection = connect_to_database();
    let value = connection.get(key);
    match value 
    {
        Err(error) => 
        {
            println!("Error at get_data_from_database, reason: {}", error);
            None
        },
        Ok(value) => value
    }
}

pub fn get_string_from_binary(binary: IVec) -> String
{
    let str = std::str::from_utf8 (&binary).unwrap();
    String::from(str)

}