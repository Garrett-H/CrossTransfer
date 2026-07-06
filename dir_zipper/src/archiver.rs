mod archiver {
    use std::io::{self, Read, Write};
    use tar::Builder;

    struct DirArchive {}

    pub fn pack<W: Write>(files: Vec<&str>, parent_path: &str, writer: W) -> io::Result<()> {
        let mut ar = Builder::new(writer);
        ar.follow_symlinks(false);

        for (i, filepath) in files.iter().enumerate() {
            let name = filepath.strip_prefix(parent_path);
            // TODO, file is not from user folder, should support later
            let name = name.ok_or_else(|| io::Error::other("TODO"))?.to_string();

            ar.append_path_with_name(filepath, &name)?;
            println!("[{}/{}] added {}", i + 1, files.len(), &name);
        }

        ar.finish()?;
        Ok(())
    }
}
