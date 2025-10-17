mod flags;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let flag = flags::gen_flag("12345678", 1);
    let check_true = flags::check_flag(&flag, "12345678", 1);
    let check_false = flags::check_flag(&flag, "87654321", 1);

    format!("Flag: {}\nCheck True: {}\nCheck False: {}", flag, check_true, check_false)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
