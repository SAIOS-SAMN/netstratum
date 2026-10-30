// Wire-format helpers for BigInt + varint serialization.
//
// Law IV exact rational: numerators and denominators encode as variable-length
// `BigInt` little-endian byte sequences with explicit sign + length
// prefixes. No fixed-width truncation; the encoder preserves the exact
// algebraic content of every coefficient.
//
// Law X chronometric chain identity: all derivation-path values that
// reach the receipt-chain hash preimage must route through these
// helpers (never via `BigInt::to_le_bytes` direct, never via
// fixed-width truncation). This is the substrate's BigInt ↔ bytes
// canonical bridge.
//
// Layout per `BigInt`: `(sign:u8, len:varint, bytes:LE)`.
// `sign` ∈ {0 = no-sign-zero, 1 = positive, 2 = negative}.
// `len` = varint-encoded byte count of the magnitude.
// `bytes` = little-endian magnitude bytes (length `len`).

use num_bigint::{BigInt, Sign};

use crate::rational::OverflowError;

/// **`encode_u64` (varint)** — encode a `u64` as 1-10 bytes using
/// unsigned LEB128 (7 bits per byte; continuation bit = 0x80).
pub fn encode_u64(mut value: u64, buf: &mut Vec<u8>) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            buf.push(byte);
            return;
        }
        buf.push(byte | 0x80);
    }
}

/// **`decode_u64` (varint)** — decode a `u64` from unsigned LEB128.
/// Returns `(value, bytes_consumed)`. Errors if the buffer ends mid-
/// varint or the value exceeds 64 bits.
pub fn decode_u64(bytes: &[u8]) -> Result<(u64, usize), OverflowError> {
    let mut value: u64 = 0;
    let mut shift: u32 = 0;
    let mut consumed: usize = 0;
    for &byte in bytes {
        let chunk = (byte & 0x7f) as u64;
        if shift >= 64 {
            return Err(OverflowError::VarintTooWide { at: consumed });
        }
        let placed = chunk << shift;
        // Any chunk bit shifted past the 64-bit width is content the
        // decode would silently drop — refused typed, never truncated
        // (Law IV; the encoder never emits this shape).
        if placed >> shift != chunk {
            return Err(OverflowError::VarintTooWide { at: consumed });
        }
        value |= placed;
        consumed += 1;
        if byte & 0x80 == 0 {
            return Ok((value, consumed));
        }
        shift += 7;
    }
    Err(OverflowError::BufferTooShort {
        needed: consumed + 1,
        actual: bytes.len(),
    })
}

/// **`write_bigint_le`** — append a `BigInt` to the buffer in the
/// canonical `(sign, varint-len, LE bytes)` layout. Exact algebraic
/// content preserved; no fixed-width truncation.
pub fn write_bigint_le(buf: &mut Vec<u8>, value: &BigInt) {
    let (sign, bytes) = value.to_bytes_le();
    let sign_byte: u8 = match sign {
        Sign::NoSign => 0,
        Sign::Plus => 1,
        Sign::Minus => 2,
    };
    buf.push(sign_byte);
    encode_u64(bytes.len() as u64, buf);
    buf.extend_from_slice(&bytes);
}

/// **`read_bigint_le`** — decode a `BigInt` from the canonical layout.
/// Returns `(value, bytes_consumed)`. Law V structured residuals at
/// every failure mode.
pub fn read_bigint_le(bytes: &[u8]) -> Result<(BigInt, usize), OverflowError> {
    if bytes.is_empty() {
        return Err(OverflowError::BufferTooShort {
            needed: 1,
            actual: 0,
        });
    }
    let sign = match bytes[0] {
        0 => Sign::NoSign,
        1 => Sign::Plus,
        2 => Sign::Minus,
        other => {
            return Err(OverflowError::InvalidSignMarker { observed: other });
        }
    };
    let mut cursor: usize = 1;
    let (len_u64, len_bytes) = decode_u64(&bytes[cursor..])?;
    cursor = cursor
        .checked_add(len_bytes)
        .ok_or(OverflowError::CursorOverflow {
            offset: cursor,
            advance: len_bytes,
        })?;
    let len = usize::try_from(len_u64).map_err(|_| OverflowError::CursorOverflow {
        offset: cursor,
        advance: 0,
    })?;
    let end = cursor
        .checked_add(len)
        .ok_or(OverflowError::CursorOverflow {
            offset: cursor,
            advance: len,
        })?;
    if end > bytes.len() {
        return Err(OverflowError::BufferTooShort {
            needed: end,
            actual: bytes.len(),
        });
    }
    let value = BigInt::from_bytes_le(sign, &bytes[cursor..end]);
    Ok((value, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn round_trip_u64(v: u64) {
        let mut buf = Vec::new();
        encode_u64(v, &mut buf);
        let (back, consumed) = decode_u64(&buf).expect("ok");
        assert_eq!(back, v);
        assert_eq!(consumed, buf.len());
    }

    #[test]
    fn varint_round_trip_small() {
        for v in [0u64, 1, 127, 128, 129, 255, 256, 1024, u32::MAX as u64] {
            round_trip_u64(v);
        }
    }

    #[test]
    fn varint_round_trip_u64_max() {
        round_trip_u64(u64::MAX);
    }

    #[test]
    fn bigint_round_trip_zero_positive_negative() {
        let big: BigInt = BigInt::from(1) << 80;
        let neg_big: BigInt = -big.clone();
        for v in [
            BigInt::from(0),
            BigInt::from(1),
            BigInt::from(-1),
            BigInt::from(127),
            BigInt::from(-127),
            big,
            neg_big,
        ] {
            let mut buf = Vec::new();
            write_bigint_le(&mut buf, &v);
            let (back, consumed) = read_bigint_le(&buf).expect("ok");
            assert_eq!(back, v, "round-trip preserves BigInt exactly");
            assert_eq!(consumed, buf.len());
        }
    }

    #[test]
    fn read_bigint_le_buffer_too_short_returns_structured_error() {
        let err = read_bigint_le(&[]).expect_err("empty buffer rejected");
        assert!(matches!(err, OverflowError::BufferTooShort { .. }));
    }

    #[test]
    fn an_invalid_sign_marker_is_refused_as_itself_never_as_a_length() {
        // The refusal carries the observed byte at its own coordinate —
        // never laundered into a fabricated buffer-length reading.
        let err = read_bigint_le(&[7, 1, 42]).expect_err("sign marker 7 rejected");
        assert_eq!(err, OverflowError::InvalidSignMarker { observed: 7 });
    }

    #[test]
    fn a_varint_carrying_content_past_the_width_is_refused_never_truncated() {
        // Ten continuation bytes place the tenth chunk at shift 63: any
        // bit above bit-0 there is content the old decode silently
        // dropped. The refusal names the byte offset.
        let overwide: [u8; 10] = [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02];
        let err = decode_u64(&overwide).expect_err("bit past width rejected");
        assert_eq!(err, OverflowError::VarintTooWide { at: 9 });
        // The exact width still decodes: bit-0 of the tenth chunk is in
        // range (u64::MAX round-trips — the canonical shape).
        let exact: [u8; 10] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
        let (value, consumed) = decode_u64(&exact).expect("exact width decodes");
        assert_eq!(value, u64::MAX);
        assert_eq!(consumed, 10);
        // An eleventh continuation byte is past the width entirely.
        let eleven: [u8; 11] = [
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01,
        ];
        let err = decode_u64(&eleven).expect_err("shift past width rejected");
        assert_eq!(err, OverflowError::VarintTooWide { at: 10 });
    }
}
