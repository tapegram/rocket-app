use rocket::get;
use rocket::launch;
use rocket::routes;

/*
* For imports, rocket examples use
* ```
* #[macro_use] extern crate rocket;
* ```
*
* Which works fine, but rust_analyzer complains about it.
*
* From some googling (this in particular:
* https://users.rust-lang.org/t/why-does-the-rocket-crate-still-require-use-of-extern-crate-re-exporting-crates/72014):
* It seems this is no longer needed and we can `use rocket`,
* we just have to import each attribute, which is probably
* what we want anyway, instead of just loading everything into scope.
*
*/

/*
* Trivial GET with no additional arguments
*/
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/*
* GET with query params
*/
#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
}

/*
* Alternative to the `launch` macro above, we can explicitly launch the rocket app
* in our main code (I think might prefer this, if there are no other differences)
*/
// fn main() {
//     rocket::ignite()
//         .mount("/", routes![index])
//         .mount("/hello", routes![hello])
//         .launch();
// }
