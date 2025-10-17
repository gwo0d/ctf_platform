mod flags;

#[macro_use]
extern crate rocket;

use flags::{check_flag, gen_flag};

#[get("/")]
fn index() -> String {
    let flag = gen_flag("12345678", 1);
    let check_true = check_flag(&flag, "12345678", 1);
    let check_false = check_flag(&flag, "87654321", 1);

    format!(
        "Flag: {}\nCheck True: {}\nCheck False: {}",
        flag, check_true, check_false
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
