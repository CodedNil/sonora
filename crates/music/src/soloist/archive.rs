//! Read the Soloist executable from Spotify's gzipped tar or zip release archives.

use std::io::{Cursor, Read as _};

use anyhow::{Context as _, Result, bail, ensure};

/// The largest decompressed archive or executable accepted from a release.
const LIMIT: u64 = 256 * 1024 * 1024;

/// Returns the regular file named `wanted`, ignoring its directory within the archive.
pub(crate) fn extract(archive: &[u8], name: &str, wanted: &str) -> Result<Vec<u8>> {
    match name.ends_with(".zip") {
        true => unzip(archive, wanted),
        false => untar(archive, wanted),
    }
}

fn untar(bytes: &[u8], wanted: &str) -> Result<Vec<u8>> {
    // Bound the whole expanded tar, not just the selected file.
    let mut expanded = Vec::new();
    flate2::read::GzDecoder::new(bytes)
        .take(LIMIT + 1)
        .read_to_end(&mut expanded)
        .context("cannot unpack the soloist archive")?;
    ensure!(
        expanded.len() as u64 <= LIMIT,
        "the soloist archive is too large"
    );
    let mut archive = tar::Archive::new(Cursor::new(expanded));
    for entry in archive
        .entries()
        .context("cannot read the soloist archive")?
    {
        let mut entry = entry.context("cannot read a soloist archive entry")?;
        if entry.header().entry_type().is_file()
            && entry
                .path()
                .context("cannot read a soloist archive path")?
                .file_name()
                .is_some_and(|name| name == wanted)
        {
            return read_entry(&mut entry);
        }
    }
    bail!("{wanted} is not in the soloist archive")
}

fn unzip(bytes: &[u8], wanted: &str) -> Result<Vec<u8>> {
    let mut archive =
        zip::ZipArchive::new(Cursor::new(bytes)).context("cannot read the soloist zip")?;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .context("cannot read a soloist zip entry")?;
        if entry.is_file()
            && entry
                .name()
                .context("cannot read a soloist zip path")?
                .rsplit('/')
                .next()
                == Some(wanted)
        {
            return read_entry(&mut entry);
        }
    }
    bail!("{wanted} is not in the soloist archive")
}

/// Bound the decompressed result even when the archive reports an incorrect size.
fn read_entry(entry: &mut impl std::io::Read) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    entry
        .take(LIMIT + 1)
        .read_to_end(&mut output)
        .context("cannot read the soloist executable")?;
    ensure!(
        output.len() as u64 <= LIMIT,
        "the soloist archive is too large"
    );
    Ok(output)
}
