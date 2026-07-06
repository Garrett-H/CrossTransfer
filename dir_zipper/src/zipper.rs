mod zipper {
    use std::io::{self, Read, Write};
    use zstd::{Decoder, stream::write::Encoder};

    pub fn zip<R: Read, W: Write>(mut reader: R, writer: W, level: i32) -> io::Result<W> {
        let mut encoder = Encoder::new(writer, level)?;
        io::copy(&mut reader, &mut encoder)?;
        let writer = encoder.finish()?;
        Ok(writer)
    }

    pub fn unzip<R: Read, W: Write>(reader: R, mut writer: W) -> io::Result<W> {
        let mut decoder = Decoder::new(reader)?;
        io::copy(&mut decoder, &mut writer)?;
        writer.flush()?;
        Ok(writer)
    }

}

#[cfg(test)]
mod tests {
    use super::zipper::{unzip, zip};

    /// Helper: compress then decompress entirely in memory, returning the result.
    fn round_trip(original: &[u8], level: i32) -> Vec<u8> {
        let compressed: Vec<u8> = zip(original, Vec::new(), level).unwrap();
        unzip(&compressed[..], Vec::new()).unwrap()
    }

    #[test]
    fn round_trip_in_memory() {
        let original = b"The quick brown fox jumps over the lazy dog. \
                         The quick brown fox jumps over the lazy dog.";

        assert_eq!(round_trip(original, 3), original);
    }

    #[test]
    fn empty_input_round_trips() {
        let original: &[u8] = b"";

        assert!(round_trip(original, 3).is_empty());
    }
}