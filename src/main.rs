#[macro_use] extern crate rocket;
extern crate tera;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::http::RawStr;

use user_handling::get_user_from_databse;
use user_handling::get_user_uuid_by_username;


mod database;
mod user_handling;

#[get("/register")]
fn register() -> Template 
{
    Template::render("index", context! {})
}

#[get("/login")]
fn login() -> Template 
{
    Template::render("login", context! {})
}

/* New user registration */
#[derive(FromForm)]
struct RegisterForm<'r> 
{
    r#name: &'r str,
    r#email: &'r str,
    r#pass: &'r str,
    r#re_pass: &'r str,
}



#[post("/register", data = "<register_form>")]
fn register_user(register_form: Form<RegisterForm<'_>>)
{
    let name = String::from(register_form.name);
    let email = String::from(register_form.email);
    let pass = String::from(register_form.pass);
    let re_pass = String::from(register_form.re_pass);
    match pass == re_pass {
        false => println!("Passwords don't match"),
        true => println!("Created user {:?}", user_handling::register_new_user(name, email, pass))
    }
    
}

/*--------------------------------------- */

#[derive(FromForm)]
struct LoginForm<'r> 
{
    r#name: &'r str,
    r#pass: &'r str,
}

#[post("/login", data = "<login_form>")]
fn login_user(login_form: Form<LoginForm<'_>>)
{
    let name = String::from(login_form.name);
    let pass = String::from(login_form.pass);
    
    let id = get_user_uuid_by_username(name);

    if id == None 
    {
        return ()
    }

    let user = get_user_from_databse(id.unwrap());

    match user.password == pass {
        false => println!("Wrong password"),
        true => println!("succesfully logged in as {:?}", user)
    }

}  



#[launch]
fn rocket() -> _ 
{
    rocket::build()
    .mount("/", routes![register, login, register_user, login_user])
    .mount("/static", FileServer::from("static_files"))
    .attach(Template::fairing())
}

