use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

const CHARSET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const KEY_LENGTH: usize = 8;
const CHECKSUM_VALUE: u16 = 1727;
const CHECKSUM_MULTIPLIER: u32 = 1664117991;
const XOR_CONSTANT: u32 = 0x3144_3772; 

pub fn generate_key() -> String {
    let mut rng = rand::rng();

    let mut data = [0u8; 18];

    initialize_data(&mut data, &mut rng);

    calculate_checksum(&mut data);

    let encoded_bytes = encode_key(&data);

    let key = format!(
        "{}-{}-{}-{}",
        std::str::from_utf8(&encoded_bytes[0..4]).unwrap(),
                      std::str::from_utf8(&encoded_bytes[4..8]).unwrap(),
                      std::str::from_utf8(&encoded_bytes[8..12]).unwrap(),
                      std::str::from_utf8(&encoded_bytes[12..16]).unwrap(),
    );
    key
}

fn get_timestamp() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time is before UNIX epoch")
        .as_nanos() as u64
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut bytes = [0u8; 8];
        let mut temp_rng = rand::rng();
        for i in 0..bytes.len() {
            bytes[i] = temp_rng.random();
        }

        u64::from_ne_bytes(bytes)
    }
}

fn initialize_data(data: &mut [u8; 18], rng: &mut impl Rng) {
    let timestamp = get_timestamp();

    let low_bits = timestamp as u32;
    let high_bits = (timestamp >> 32) as u32;

    let seed = low_bits.wrapping_mul(high_bits);

    let seed_bytes = seed.to_le_bytes();
    data[0] = seed_bytes[2] ^ 1;
    data[1] = seed_bytes[3];
    data[2] = seed_bytes[0];
    data[3] = seed_bytes[1];

    let second = 2u32.wrapping_mul(seed ^ XOR_CONSTANT);
    let second_bytes = second.to_le_bytes();
    data[4] = second_bytes[0];
    data[5] = second_bytes[1];
    data[6] = second_bytes[2];
    data[7] = second_bytes[3];

    for i in (KEY_LENGTH + 2)..data.len() {
        data[i] = rng.random();
    }
}

fn calculate_checksum(data: &mut [u8; 18]) {
    let mut checksum: u32 = 0;
    for i in 0..KEY_LENGTH {
        checksum = (data[i] as u32)
        .wrapping_sub(CHECKSUM_MULTIPLIER.wrapping_mul(checksum));
    }
    let checksum_word = CHECKSUM_VALUE ^ ((checksum % 0xFFF1) as u16);
    data[KEY_LENGTH] = (checksum_word & 0xFF) as u8;
    data[KEY_LENGTH + 1] = (checksum_word >> 8) as u8;
}

fn encode_key(data: &[u8; 18]) -> [u8; 30] {
    let mut encoded = [0u8; 30];
    let mut output_index = 0;
    let mut offset_bit = 0u8;

    for chunk_start in (0..data.len()).step_by(3) {
        let b0 = data[chunk_start];
        let b1 = *data.get(chunk_start + 1).unwrap_or(&0);
        let b2 = *data.get(chunk_start + 2).unwrap_or(&0);
        let b3 = *data.get(chunk_start + 3).unwrap_or(&0);
        let chunk = u32::from_le_bytes([b0, b1, b2, b3]);

        let mut bit_position = offset_bit;
        for _ in 0..5 {
            let index = ((chunk >> bit_position) & 0x1F) as usize;
            encoded[output_index] = CHARSET.as_bytes()[index];
            output_index += 1;
            bit_position = bit_position.wrapping_add(5);
        }
        offset_bit = offset_bit.wrapping_add(1);
    }

    encoded
}
