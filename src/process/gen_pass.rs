use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("Upper won't be empty"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("Lower won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("Number won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("Symbol won't be empty"));
    }
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in the context");
        password.push(*c);
    }
    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;
    println!("{}", password);

    // output the password strength in stderr
    let estimate = zxcvbn(&password, &[]);
    eprint!("Password Strength: {}", estimate.score());
    Ok(())
}
