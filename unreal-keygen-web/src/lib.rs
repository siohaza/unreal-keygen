use wasm_bindgen::prelude::*;
use unreal_keygen_core::{GameVersion, generate_key as core_generate_key};

#[wasm_bindgen]
pub fn generate_key(game_type: u8) -> String {
    let version = match game_type {
        0 => GameVersion::UT2003,
        1 => GameVersion::UT2004,
        2 => GameVersion::UT3,
        _ => GameVersion::UT2004,
    };

    core_generate_key(version)
}
