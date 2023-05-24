use anyhow::*;
use fs_extra::copy_items;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=res/*");

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["res/"];

    let out_dir = std::env::var("OUT_DIR")?;
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
