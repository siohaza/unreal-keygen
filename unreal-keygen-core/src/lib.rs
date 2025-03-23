pub mod ut2k_common;
pub mod ut2k3;
pub mod ut2k4;
pub mod ut3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVersion {
    UT2003,
    UT2004,
    UT3,
}

pub fn generate_key(version: GameVersion) -> String {
    match version {
        GameVersion::UT2003 => ut2k3::generate_key(),
        GameVersion::UT2004 => ut2k4::generate_key(),
        GameVersion::UT3 => ut3::generate_key(),
    }
}
