use crate::CONFIG;

use {
    don_error::*,
    imagesize::size,
    std::{
        fs::{create_dir_all, rename},
        path::{Path, PathBuf},
    },
};

const RATIO_LIMIT: f64 = 16.0 / 9.0 * 1.3;

pub fn perform(force_sort_all_wallpapers: bool) -> DonResult<()> {
    let wallpapers_path = PathBuf::from(&CONFIG.wallpapers_dir);
    if !wallpapers_path.exists() {
        bail!("{} not found on this computer", &CONFIG.wallpapers_dir);
    }
    let single_dir = wallpapers_path.clone().join(&CONFIG.single_screen_dir);
    if !single_dir.exists() {
        create_dir_all(&single_dir)?;
    }
    let dual_dir: PathBuf = wallpapers_path.clone().join(&CONFIG.dual_screen_dir);
    if !dual_dir.exists() {
        create_dir_all(&dual_dir)?;
    }

    if force_sort_all_wallpapers {
        move_all_files(&single_dir, &wallpapers_path)?;
        move_all_files(&dual_dir, &wallpapers_path)?;
    }

    for img_path in get_files(&wallpapers_path)? {
        if img_path.ends_with("Thumbs.db") {
            continue;
        }
        try_or_report(|| {
            let img_dimensions = size(&img_path)
                .map_err(|err| err_msg!("Problem with img {img_path:#?} : {err:#?}"))?;
            if img_dimensions.width as f64 / img_dimensions.height as f64 <= RATIO_LIMIT {
                move_to(&img_path, &single_dir)?;
            } else {
                move_to(&img_path, &dual_dir)?;
            };
            Ok(())
        })
    }

    Ok(())
}

fn get_files(dir: &Path) -> DonResult<Vec<PathBuf>> {
    Ok(dir
        .read_dir()?
        .map(|img| Ok(img?.path()))
        .collect::<DonResult<Vec<_>>>()?
        .into_iter()
        .filter(|img| img.is_file())
        .collect())
}

fn move_to(file_path: &Path, new_dir: &Path) -> DonResult<()> {
    let mut new_path = new_dir.to_path_buf();
    new_path.push(
        file_path
            .file_name()
            .expect("images all have valid filename"),
    );
    Ok(rename(file_path, new_path)?)
}

fn move_all_files(old_dir: &Path, new_dir: &Path) -> DonResult<()> {
    for img_path in get_files(old_dir)? {
        move_to(&img_path, new_dir)?;
    }
    Ok(())
}
