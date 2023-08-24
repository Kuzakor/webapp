/* Rocket & Tera */
#[macro_use] extern crate rocket;
extern crate tera;
/* ------------ */

/* Rocket libralies */
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::fs::FileServer;
use rocket::form::Form;
/* ------------- */


/* server rust files */
mod database;
mod user_handling;
/* --------------- */


/* Static sites */
#[get("/register")]
fn register() -> Template 
{
    Template::render("register", context! {})
}

#[get("/login")]
fn login() -> Template 
{
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
fn login_user(login_form: Form<LoginForm<'_>>) -> Template
{
    let name = String::from(login_form.name);
    let pass = String::from(login_form.pass);
    
    let id = user_handling::get_user_uuid_by_username(name);

    if id == None 
    {
        return Template::render("login_register_response", context! {formtype:"Sign in", response: "No such user"})
    }

    let user = user_handling::get_user_from_databse(id.unwrap());

    match user.password == pass 
    {
        false => Template::render("login_register_response", context! {formtype:"Sign in", response: "Wrong password"}),
        true => Template::render("login_register_response", context! {formtype:"Sign in", response: "Succesfully logged in"})
    }

}  
/* -------------------------------------------- */

/* Launching the server */
#[launch]
fn rocket() -> _ 
{
    rocket::build()
    .mount("/", routes![register, login, register_user, login_user])
    .mount("/static", FileServer::from("static_files"))
    .attach(Template::fairing())
}
/* ---------------- */
