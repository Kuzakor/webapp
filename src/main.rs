/* Rocket & Tera */
#[macro_use]
extern crate rocket;
extern crate tera;
/* ------------ */

/* Rocket libralies */
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::hyper::header::TE;
use rocket::http::{Cookie, CookieJar};
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;
/* ------------- */

/* server rust files */
mod database;
mod user_handling;
/* --------------- */

/* Static sites */

/* Index */
#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    if let Some(user) = logged_in_user(cookies) {
        return Template::render("index", context! {
            button_post_request: "/search_for_game",
            subtitle: "Logged in as".to_owned() + &user.username,
            button_name: "Search for a new game",
            login_button_get_request: "/logout",
            login_button: "Logout"
        });
    }
    Template::render("index", context! {
        button_post_request: "/register",
        subtitle: "An online trading game",
        button_name: "Get started",
        login_button_get_request: "/login",
        login_button: "Login"
    })
}
/* ------ */

/* Register */
#[get("/register")]
fn register() -> Template {
    Template::render("register", context! {})
}
/* --------- */

/* Login */
#[get("/login")]
fn login(cookies: &CookieJar<'_>) -> Template {
    if let Some(user) = logged_in_user(cookies) {
        return Template::render(
            "login_register_response",
            context! {formtype: "Log in", response: "Already logged in as ".to_owned() + &user.username},
        );
    }
    Template::render("login", context! {})
}
/* -------------- */

/* Log out */
#[get("/logout")] 
fn logout(cookies: &CookieJar<'_>) -> Template{
    cookies.remove(Cookie::named("uuid"));
    Template::render("index", context! {
        button_post_request: "/register",
        subtitle: "Succesfully logged out",
        button_name: "Get started",
        login_button_get_request: "/login",
        login_button: "Login"
    })
}
/* -------------- */

/* Look for a game */
#[get("/search_for_game")]
fn search_for_a_game() -> Template {
    Template::render("game_looking", context!{})

}



/* ---------------------------- */

/* New user registration */
#[derive(FromForm)]
struct RegisterForm<'r> {
    r#name: &'r str,
    r#email: &'r str,
    r#pass: &'r str,
    r#re_pass: &'r str,
}

#[post("/register", data = "<register_form>")]
fn register_user(register_form: Form<RegisterForm<'_>>) -> Template {
    let name = String::from(register_form.name);
    let email = String::from(register_form.email);
    let pass = String::from(register_form.pass);
    let re_pass = String::from(register_form.re_pass);

    if pass != re_pass {
        return Template::render(
            "login_register_response",
            context! {formtype:"Sign up",  response: "Passwords don't match"},
        );
    }

    user_handling::register_new_user(name, email, pass);

    Template::render(
        "login_register_response",
        context! {formtype:"Sign up", response: "Succesfully registered new account"},
    )
}

/*--------------------------------------- */

/* Logining in */
#[derive(FromForm)]
struct LoginForm<'r> {
    r#name: &'r str,
    r#pass: &'r str,
}

#[post("/login", data = "<login_form>")]
fn login_user(login_form: Form<LoginForm<'_>>, cookies: &CookieJar<'_>) -> Template {
    let name = String::from(login_form.name);
    let pass = String::from(login_form.pass);
    let id = user_handling::get_user_uuid_by_username(name);

    if id.is_none() {
        return Template::render(
            "login_register_response",
            context! {formtype:"Log in", response: "No such user"},
        );
    }

    let user = user_handling::get_user_from_databse(id.unwrap());

    if user.password != pass {
        return Template::render(
            "login_register_response",
            context! {formtype:"Log in", response: "Wrong password"},
        );
    }

    let cookie = Cookie::build("uuid", user.uuid).path("/").finish();
    cookies.add(cookie);
    Template::render(
        "login_register_response",
        context! {formtype:"Log in", response: "Succesfully logged in"},
    )
}

/* -------------------------------------------- */


/* Non-web functions */

pub fn logged_in_user(cookies: &CookieJar<'_>) -> Option<user_handling::User>{
    let cookie = cookies.get("uuid");
    if let Some(cookie) = cookie {
        return Some(user_handling::get_user_from_databse(String::from(cookie.value())));
    }
    None
}


/* -------------- */


/* Launching the server */
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, register, login, logout, search_for_a_game, register_user, login_user],
        )
        .mount("/static", FileServer::from("static_files"))
        .attach(Template::fairing())
}
/* ---------------- */
