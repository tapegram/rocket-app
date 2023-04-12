use rocket::get;
use rocket::launch;
use rocket::post;
use rocket::routes;
use rocket::serde::{
    Deserialize, 
    json::Json, Serialize
};

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

/*
* POST with JSON body
https://rocket.rs/v0.5-rc/guide/requests/#json
 */
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    complete: bool
}

#[post("/todo", data = "<task>")]
fn new(task: Json<Task<'_>>) { 
    format!("You posted a task with: {}, {}", task.description, task.complete);
}

/*
https://rocket.rs/v0.5-rc/guide/responses/#responder
https://rocket.rs/v0.4/guide/responses/#json
 */
#[derive(Serialize)]
struct Person {
    name: &'static str,
    title: &'static str,
}

#[get("/")]
fn json() -> Json<Person> {
    Json(Person {name: "John", title: "Captain" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .mount("/tasks", routes![new])
        .mount("/json", routes![json])
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
