// F2FS image pack command.
// Packs a directory into an F2FS filesystem image.

use anyhow::Result;
use std::path::PathBuf;

// Pack an F2FS image
#[allow(clippy::too_many_arguments)]
pub fn run_f2fs_pack(
    source: &str,
    output: &str,
    size: &str,
    mount_point: &str,
    file_contexts: Option<String>,
    fs_config: Option<String>,
    sparse: bool,
    label: Option<String>,
    readonly: bool,
    project_quota: bool,
    casefold: bool,
    compression: bool,
    root_uid: u32,
    root_gid: u32,
    timestamp: Option<u64>,
) -> Result<()> {
    use crate::container::sparse::convert_to_sparse;
    use crate::filesystem::f2fs::consts::F2FS_BLKSIZE;
    use crate::filesystem::f2fs::types::{F2fsBuilderConfig, F2fsFeatures};
    use crate::filesystem::f2fs::write::F2fsBuilder;

    // Parse image size
    let image_size = super::parse_size(size)?;

    log::info!("source: {}", source);
    log::info!("output: {}", output);
    log::info!(
        "image size: {} bytes ({:.2} MB)",
        image_size,
        image_size as f64 / 1024.0 / 1024.0
    );

    // Build feature flags.
    // inode_chksum and sb_chksum are disabled until basic functionality is verified.
    let features = F2fsFeatures {
        readonly,
        project_quota,
        casefold,
        compression,
        extra_attr: false,
        inode_chksum: false,
        sb_chksum: false,
        ..Default::default()
    };

    // Build config
    let config = F2fsBuilderConfig {
        source_dir: PathBuf::from(source),
        output_path: PathBuf::from(output),
        image_size,
        mount_point: mount_point.to_string(),
        file_contexts: file_contexts.map(PathBuf::from),
        fs_config: fs_config.map(PathBuf::from),
        sparse_mode: sparse,
        features,
        compression: None,
        volume_label: label.unwrap_or_default(),
        root_uid,
        root_gid,
        timestamp,
    };

    // Create builder and build
    let mut builder = F2fsBuilder::new(config)?;
    builder.build()?;

    // Builder only emits raw image; convert to sparse here as a post-processing step.
    // On failure, keep the raw temp so the user does not lose the freshly built image.
    if sparse {
        let raw_tmp = format!("{}.raw.{}.tmp", output, std::process::id());
        std::fs::rename(output, &raw_tmp)?;
        match convert_to_sparse(
            std::path::Path::new(&raw_tmp),
            std::path::Path::new(output),
            F2FS_BLKSIZE as u32,
        ) {
            Ok(()) => {
                let _ = std::fs::remove_file(&raw_tmp);
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "convert_to_sparse failed: {}; raw image preserved at {}",
                    e,
                    raw_tmp
                ));
            }
        }
    }

    log::info!("F2FS image built: {}", output);
    Ok(())
}
