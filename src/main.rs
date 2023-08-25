/* Rocket & Tera */
#[macro_use] extern crate rocket;
extern crate tera;
/* ------------ */

/* Rocket libralies */
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::fs::FileServer;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
/* ------------- */


/* server rust files */
mod database;
mod user_handling;
/* --------------- */

/* other libarlies */
use chrono::{Utc, Duration};
/*---------- */



/* Static sites */

/* Index */
#[get("/")]
fn index() -> Template 
{
    Template::render("index", context! {})
}
/* ------ */

/* Register */
#[get("/register")]
fn register() -> Template 
{
    Template::render("register", context! {})
}
/* --------- */

/* Login */
#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> Template 
{
    let cookie = cookies.get("uuid");
    if cookie != None 
    {
        let logged_in_username = user_handling::get_user_from_databse(String::from(cookie.unwrap().value()));
        println!("{}", logged_in_username.username);
        return Template::render("login_register_response", context! {formtype: "Log in", response: "Already logged in as ".to_owned() + &logged_in_username.username})
    }
    Template::render("login", context! {})
}
/* ---------------------------- */

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
fn register_user(register_form: Form<RegisterForm<'_>>) -> Template
{
    let name = String::from(register_form.name);
    let email = String::from(register_form.email);
    let pass = String::from(register_form.pass);
    let re_pass = String::from(register_form.re_pass);

    if !(pass == re_pass) 
    {
        return Template::render("login_register_response", context! {formtype:"Sign up",  response: "Passwords don't match"});
    }
    
    user_handling::register_new_user(name, email, pass);
    
    Template::render("login_register_response", context! {formtype:"Sign up", response: "Succesfully registered new account"})
}

/*--------------------------------------- */

/* Logining in */
#[derive(FromForm)]
struct LoginForm<'r> 
{
    r#name: &'r str,
    r#pass: &'r str,
}

#[post("/login", data = "<login_form>")]
fn login_user(login_form: Form<LoginForm<'_>>, cookies: &CookieJar<'_>) -> Template
{
    let name = String::from(login_form.name);
    let pass = String::from(login_form.pass);
    let id = user_handling::get_user_uuid_by_username(name);

    if id == None 
    {
        return Template::render("login_register_response", context! {formtype:"Log in", response: "No such user"})
    }

    let user = user_handling::get_user_from_databse(id.unwrap());

    if !(user.password == pass) {
        return Template::render("login_register_response", context! {formtype:"Log in", response: "Wrong password"})
    }

    let cookie = Cookie::build("uuid", user.uuid)
    .path("/")
    .finish();
    println!("{:?}", cookie.clone().value());
    
    cookies.add(cookie);
    Template::render("login_register_response", context! {formtype:"Log in", response: "Succesfully logged in"})
    

}  
/* -------------------------------------------- */

/* Launching the server */
#[launch]
fn rocket() -> _ 
{
    rocket::build()
    .mount("/", routes![index, register, login, register_user, login_user])
    .mount("/static", FileServer::from("static_files"))
    .attach(Template::fairing())
}
/* ---------------- */
