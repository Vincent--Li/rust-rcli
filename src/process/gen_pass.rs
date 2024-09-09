use rand::{seq::SliceRandom, thread_rng};
use zxcvbn::zxcvbn;

pub fn process_genpass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: usize,
) -> anyhow::Result<String> {
    let mut rng = thread_rng();
    let mut password = Vec::with_capacity(length);
    let mut chars = Vec::new();

    if uppercase {
        chars.extend('A'..='Z');
    }

    if lowercase {
        chars.extend('a'..='z');
    }

    if number {
        chars.extend('0'..='9');
    }

    if symbol {
        chars.extend("!@#$%^&*_".chars());
    }

    for _ in 0..length {
        let random_char = chars.choose(&mut rng).unwrap();
        password.push(*random_char as u8);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    // output password strength in stderr
    let result = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", result.score());

    Ok(password)
}
