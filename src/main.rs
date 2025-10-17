mod flags;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    flags::gen_flag("12345678", 1)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
