use crate::CONFIG;

use {
    don_error::*,
    imagesize::size,
    std::{
        fs::{create_dir_all, rename},
        path::{Path, PathBuf},
    },
    walkdir::WalkDir,
};

const RATIO_LIMIT: f64 = 16.0 / 9.0 * 1.3;

pub fn perform(force_sort_all_wallpapers: bool) -> DonResult<()> {
    let wallpapers_path = PathBuf::from(&CONFIG.wallpapers_dir);
    if !wallpapers_path.exists() {
        bail!("{} not found on this computer", &CONFIG.wallpapers_dir);
    }
    let single_dir = wallpapers_path.join(&CONFIG.single_screen_dir);
    if !single_dir.exists() {
        create_dir_all(&single_dir)?;
    }
    let dual_dir: PathBuf = wallpapers_path.join(&CONFIG.dual_screen_dir);
    if !dual_dir.exists() {
        create_dir_all(&dual_dir)?;
    }

    if force_sort_all_wallpapers {
        move_all_files(&single_dir, &wallpapers_path)?;
        move_all_files(&dual_dir, &wallpapers_path)?;
    }

    get_file_paths(&wallpapers_path)
        .filter(|img_path| !img_path.ends_with("Thumbs.db"))
        .for_each(|img_path| {
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
        });

    Ok(())
}

fn get_file_paths(dir: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(dir).into_iter().filter_map(|e| {
        e.ok()
            .and_then(|e| e.path().is_file().then(|| e.path().to_owned()))
    })
}

fn move_to(file_path: &Path, new_dir: &Path) -> Result<(), std::io::Error> {
    rename(
        file_path,
        new_dir.join(
            file_path
                .file_name()
                .expect("images all have valid filename"),
        ),
    )
}

fn move_all_files(old_dir: &Path, new_dir: &Path) -> DonResult<()> {
    for img_path in get_file_paths(old_dir) {
        move_to(&img_path, new_dir)?;
    }
    Ok(())
}
