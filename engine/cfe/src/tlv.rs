//! The additive TLV wire (Charter SEAL A). One record is
//! `tag u8 ‖ LE32(length) ‖ value bytes`. A reader iterates records and
//! skips tags it does not know — unknown content is future content, not
//! an error. Tags are permanent: one tag, one meaning, forever.

/// Append one record.
pub fn put(tag: u8, value: &[u8], out: &mut Vec<u8>) {
    out.push(tag);
    out.extend_from_slice(&(value.len() as u32).to_le_bytes());
    out.extend_from_slice(value);
}

/// Iterate `(tag, value)` records; a truncated tail ends iteration.
pub fn records(bytes: &[u8]) -> Records<'_> {
    Records { bytes, at: 0 }
}

pub struct Records<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Iterator for Records<'a> {
    type Item = (u8, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        let tag = *self.bytes.get(self.at)?;
        let len_bytes = self.bytes.get(self.at + 1..self.at + 5)?;
        let len = u32::from_le_bytes(len_bytes.try_into().ok()?) as usize;
        let value = self.bytes.get(self.at + 5..self.at + 5 + len)?;
        self.at += 5 + len;
        Some((tag, value))
    }
}
