mod archiver {
    use std::io::{self, Read, Write};
    use tar::Builder;
    use zstd::stream::raw::CParameter;
    use zstd::{Decoder, stream::write::Encoder};

    struct ZipperConfig<'a, W: Write> {
        pub files: &'a mut Vec<String>,
        pub parent_path: String,
        pub writer: W,
        pub level: u8,
        pub win_log: Option<u32>,
    }

    pub fn pack<'a, W:Write>(config: ZipperConfig<'a, W>) -> io::Result<()> {
        let mut encoder = Encoder::new(config.writer, config.level.into())?;
        // windowLog = 2^n -> 27==128MiB
        encoder.set_parameter(CParameter::WindowLog(config.win_log.unwrap_or(27)))?;
        // For matches that are far apart
        encoder.set_parameter(CParameter::EnableLongDistanceMatching(true))?;

        let mut archiver = Builder::new(encoder);
        archiver.follow_symlinks(false);

        log::debug!("Beginning archival");
        for (i, filepath) in config.files.iter().enumerate() {
            let name = filepath.strip_prefix(config.parent_path.as_str());
            // TODO, file is not from user folder, should support later
            let name = name.ok_or_else(|| io::Error::other("TODO"))?.to_string();

            archiver.append_path_with_name(filepath, &name)?;
            log::info!("[{}/{}] added {}", i + 1, config.files.len(), &name);
        }
        log::debug!("Finished archival");

        // Pulls encoder back out & finish
        (archiver.into_inner()?).finish();
        Ok(())
    }

    pub fn unzip<R: Read, W: Write>(
        reader: R,
        mut writer: W,
        win_log: Option<u32>,
    ) -> io::Result<W> {
        let mut decoder = Decoder::new(reader)?;
        // Must be >= to encoder's windowLog
        decoder.set_parameter(zstd::zstd_safe::DParameter::WindowLogMax(
            win_log.unwrap_or(27),
        ))?;

        io::copy(&mut decoder, &mut writer)?;
        Ok(writer)
    }
}

#[cfg(test)]
mod tests {
    // use super::archiver::{unzip, pack};

    // /// Helper: compress then decompress entirely in memory, returning the result.
    // fn round_trip(original: &[u8], level: i32, win_log: Option<u32>) -> Vec<u8> {
    //     let compressed: Vec<u8> = pack(original, Vec::new(), level, win_log).unwrap();
    //     unzip(&compressed[..], Vec::new(), win_log).unwrap()
    // }

    // #[test]
    // fn round_trip_in_memory() {
    //     let original = b"The quick brown fox jumps over the lazy dog. \
    //                      The quick brown fox jumps over the lazy dog.";

    //     assert_eq!(round_trip(original, 3, None), original);
    // }

    // #[test]
    // fn empty_input_round_trips() {
    //     let original: &[u8] = b"";

    //     assert!(round_trip(original, 3, None).is_empty());
    // }
}
