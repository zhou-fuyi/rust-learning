use rand::{seq::SliceRandom, thread_rng};

const UPPER: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*-_";


/// 随机生成密码的需求：长度，大小写，数字，特殊字符，强度
/// 随机数生成：rand crate
/// 构建一个密码生成器
/// 密码强度检测：zxcvbn crate

pub fn process_genpass(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> anyhow::Result<String> {

    let mut rng = thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("No uppercase characters available for password generation"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("No lowercase characters available for password generation"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("No numeric characters available for password generation"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("No symbol characters available for password generation"));
    }

    for _ in 0..length - password.len() as u8 {
        // Randomly choose a character from the available characters
        // Ensure that we have at least one of each type already in the password
        let c = chars.choose(&mut rng).expect("No characters available for password generation");
        password.push(*c);
    }
    // Shuffle the password to ensure randomness

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    Ok(password)
}
