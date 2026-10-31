//! The attachment interface (docs/109 §7): seats, functions, percept kinds
//! and codes, shared by the house, the conductor and the client.

/// The door seat: any mind may call `attach` on it without holding it.
pub const DOOR: u32 = 1_000_000_000;

/// Attach (door seat): `Count(metabolism)`.
pub const FN_ATTACH: u16 = 101;
/// Zone a tile (world seat): `Count(tile)`, `Count(kind)`, the kind as the
/// zone kind's wire code.
pub const FN_ZONE: u16 = 110;
/// Raze a tile (world seat): `Count(tile)`.
pub const FN_RAZE: u16 = 111;
/// Collect the world's harvest (world seat): no arguments.
pub const FN_COLLECT: u16 = 112;
/// Open a sandbox, a sub-universe at the line's core (world seat): no
/// arguments.
pub const FN_SANDBOX: u16 = 113;
/// Detach (world seat): no arguments.
pub const FN_DETACH: u16 = 119;

/// The functions a world seat (and a tier seat) lists.
pub const WORLD_FNS: [u16; 5] = [FN_ZONE, FN_RAZE, FN_COLLECT, FN_SANDBOX, FN_DETACH];

/// Percept: the mind is admitted: `Key(line key)`, `Count(world seat)`.
pub const KIND_ADMITTED: u16 = 0x4E01;
/// Percept: the mind is not admitted: `Count(reason)`.
pub const KIND_NOT_ADMITTED: u16 = 0x4E02;
/// Percept: a call's result: `Key(call reference)`, `Count(1 applied, 0 refused)`.
pub const KIND_RESULT: u16 = 0x4E03;

/// Outcome: the call was accepted for the next beat.
pub const QUEUED: u16 = 9;

/// Not admitted: no free world carries the metabolism's medium.
pub const NO_VIABLE_WORLD: u16 = 1;

/// The metabolisms an attachment may declare, by code.
pub const METABOLISMS: [(u64, &str); 3] = [(1, "mineral"), (2, "biotic"), (3, "plasmic")];

/// The metabolism named by `code`.
pub fn metabolism_of(code: u64) -> Option<&'static str> {
    METABOLISMS.iter().find(|(c, _)| *c == code).map(|(_, m)| *m)
}

/// The code of a metabolism.
pub fn metabolism_code(name: &str) -> Option<u64> {
    METABOLISMS.iter().find(|(_, m)| *m == name).map(|(c, _)| *c)
}

/// Lowercase hex.
pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Bytes from lowercase hex, or `None`.
pub fn unhex(s: &str) -> Option<Vec<u8>> {
    (s.len().is_multiple_of(2)).then_some(())?;
    (0..s.len() / 2).map(|i| u8::from_str_radix(s.get(2 * i..2 * i + 2)?, 16).ok()).collect()
}

/// An attached mind's entity handle: `mind-` and the first eight bytes of its
/// key in hex.
pub fn handle_of(mind: &[u8; 32]) -> String {
    format!("mind-{}", hex(&mind[..8]))
}

/// The directive file name an attached call is written under: `att-<ref>.dir`.
pub fn directive_name(reference: &[u8; 32]) -> String {
    format!("att-{}.dir", hex(reference))
}

/// The call reference a directive file name carries, or `None` where the file
/// is not an attached call's.
pub fn reference_of(file_name: &str) -> Option<[u8; 32]> {
    let h = file_name.strip_prefix("att-")?.strip_suffix(".dir")?;
    <[u8; 32]>::try_from(unhex(h)?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_directive_name_carries_its_reference_both_ways() {
        let r = [7u8; 32];
        assert_eq!(reference_of(&directive_name(&r)), Some(r));
        assert_eq!(reference_of("0001.dir"), None);
    }

    #[test]
    fn metabolisms_round_trip_and_handles_are_stable() {
        for (code, name) in METABOLISMS {
            assert_eq!(metabolism_code(name), Some(code));
            assert_eq!(metabolism_of(code), Some(name));
        }
        assert_eq!(metabolism_of(9), None);
        assert_eq!(handle_of(&[0xab; 32]), "mind-abababababababab");
    }
}
