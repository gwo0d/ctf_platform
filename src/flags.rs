use argon2::Argon2;
use std::env;

pub fn gen_flag(student_number: &str, challenge_number: u8) -> String {
    let token_secret =
        env::var("TOKEN_SECRET").expect("No `TOKEN_SECRET` value set in environment.");

    let pwd = format!("{student_number}{challenge_number}");
    let pwd = pwd.as_bytes();
    let salt = token_secret.as_bytes();

    let hasher = Argon2::default();

    let mut output = [0u8; 8];

    hasher
        .hash_password_into(pwd, salt, &mut output)
        .expect("Failed to generate Argon2 hash.");

    let hex_digest = hex::encode(output);

    format!("flag{{{hex_digest}}}")
}

pub fn check_flag(flag: &str, student_number: &str, challenge_number: u8) -> bool {
    flag == gen_flag(student_number, challenge_number)
}
