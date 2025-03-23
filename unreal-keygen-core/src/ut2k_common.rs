use md5::{Digest, Md5};
use rand::Rng;

const SCRAMBLE_TABLE: &str = "ABCDEFGHJLKMNPQRTUVWXYZ2346789";

pub fn generate_key_with_magic(magic: &str) -> String {
    let mut rng = rand::rng();
    let seed: u64 = rng.random();

    let key = encode_seed(seed);

    let checksum = calculate_checksum(seed, magic);

    format_key(key, checksum)
}

fn encode_seed(seed: u64) -> String {
    let base30 = to_base_n(seed, 30);

    let scrambled = scramble(&base30);
    pad_left(&scrambled, 14, 'A')
}

fn calculate_checksum(seed: u64, magic: &str) -> String {
    let check_data = format!("{}{}", seed as i64, magic);
    let mut hasher = Md5::new();
    hasher.update(check_data.as_bytes());
    let digest = hasher.finalize();

    let mut digest_bytes = [0u8; 8];
    digest_bytes.copy_from_slice(&digest[0..8]);
    let digest_u64 = u64::from_le_bytes(digest_bytes);

    let base30 = to_base_n(digest_u64, 30);
    let scrambled = scramble(&base30);
    pad_left(&scrambled, 6, 'A')
}

fn format_key(key: String, checksum: String) -> String {
    let fullkey = format!("{}{}", key, checksum);
    format!(
        "{}-{}-{}-{}",
        &fullkey[10..15],
        &fullkey[5..10],
        &fullkey[0..5],
        &fullkey[15..20]
    )
}

fn to_base_n(mut number: u64, base: u8) -> String {
    if base < 2 || base > 36 {
        panic!("Base must be between 2 and 36");
    }

    if number == 0 {
        return "0".to_string();
    }

    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";
    let mut result = String::new();

    while number > 0 {
        let digit_index = (number % base as u64) as usize;
        result.push(digits.chars().nth(digit_index).unwrap());
        number /= base as u64;
    }

    result.chars().rev().collect()
}

fn scramble(text: &str) -> String {
    let mut result = String::with_capacity(text.len());

    for ch in text.chars() {
        let code = ch as u32;

        let index = match code {
            0x30..=0x39 => code - 0x30,
            0x41..=0x5A => code - 0x41 + 10,
            0x61..=0x7A => code - 0x61 + 10,
            _ => {
                result.push('\0');
                continue;
            }
        };

        if index < SCRAMBLE_TABLE.len() as u32 {
            result.push(SCRAMBLE_TABLE.chars().nth(index as usize).unwrap());
        }
    }

    result
}

fn pad_left(text: &str, width: usize, pad_char: char) -> String {
    if text.len() >= width {
        return text.to_string();
    }

    let padding_length = width - text.len();
    let mut result = String::with_capacity(width);

    for _ in 0..padding_length {
        result.push(pad_char);
    }

    result.push_str(text);
    result
}
