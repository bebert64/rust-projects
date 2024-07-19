use crate::{
    monitors::{screens_config, ScreensConfig},
    CONFIG,
};

use {
    clap::ValueEnum,
    don_error::{bail, try_or_report, DonResult, DonResultOptionExtensions},
    rand::Rng,
    std::{
        env::var,
        fs::{create_dir_all, File},
        path::PathBuf,
        process::{Command, Stdio},
        thread::sleep,
        time::Duration,
    },
};

#[derive(ValueEnum, Clone, Debug)]
pub enum Mode {
    OnlySingle,
    OnlyDual,
    ProportionateToNumberOfFiles,
    FiftyFifty,
}

pub fn once(mode: &Mode) -> DonResult<()> {
    use {Mode::*, ScreensConfig::*};
    match &screens_config()? {
        SingleScreen | DualScreenDifferentResolution | ThreeScreensOrMore => choose_single(),
        DualScreenSameResolution => match *mode {
            OnlySingle => choose_single(),
            OnlyDual => choose_dual(),
            FiftyFifty => choose_single_or_dual_randomly(0.5),
            ProportionateToNumberOfFiles => {
                // We'll select 2 single wallpapers each time we run the 'feh' command, so we divide
                // by 2 to get a similar probability for each wallpaper.
                //
                // Ex : if we have 10 single wallpapers and 2 dual ones, for each to be selected
                // once, we'll need to select 2 singles * 5 times + each dual once.
                // So we need a proba of 5 out of 7 for Single and 2 out of 7 for Dual.
                let nb_single_divided_by_two = PathBuf::from(&CONFIG.wallpapers_dir)
                    .join(&CONFIG.single_screen_dir)
                    .read_dir()?
                    .count() as f64
                    / 2.;
                let nb_dual = PathBuf::from(&CONFIG.wallpapers_dir)
                    .join(&CONFIG.dual_screen_dir)
                    .read_dir()?
                    .count() as f64;
                choose_single_or_dual_randomly(
                    nb_single_divided_by_two / (nb_single_divided_by_two + nb_dual),
                )
            }
        },
    }
}

pub fn every_n_min(minutes: u64, mode: &Mode) -> DonResult<()> {
    let lock_file_path = PathBuf::from(format!("{}/.wallpapers-mgr/lock", var("HOME")?));
    if !lock_file_path.exists() {
        create_dir_all(
            lock_file_path
                .parent()
                .ok_or_don_err("just created the path with a parent")?,
        )?;
        File::create(&lock_file_path)?;
    }
    let mut lock_file = fd_lock::RwLock::new(File::open(&lock_file_path)?);
    let lock = lock_file.try_write();
    // If the lock is taken, it means a cron is already running.
    // In that case, we still want to change the wallpapers once but not start a second cron.
    match lock {
        Ok(_) => loop {
            try_or_report(|| once(mode));
            sleep(Duration::new(minutes * 60, 0));
        },
        Err(_) => once(mode),
    }
}

fn choose_single_or_dual_randomly(probability_to_chose_single_wallpapers: f64) -> DonResult<()> {
    if rand::thread_rng().gen::<f64>() <= probability_to_chose_single_wallpapers {
        choose_single()
    } else {
        choose_dual()
    }
}

fn choose_single() -> DonResult<()> {
    run_feh_command(&[
        "--bg-max",
        "--no-fehbg",
        "--random",
        &format!("{}/{}", CONFIG.wallpapers_dir, CONFIG.single_screen_dir),
    ])
}

fn choose_dual() -> DonResult<()> {
    run_feh_command(&[
        "--bg-max",
        "--no-fehbg",
        "--no-xinerama",
        "--random",
        &format!("{}/{}", CONFIG.wallpapers_dir, CONFIG.dual_screen_dir),
    ])
}

fn run_feh_command(args: &[&str]) -> DonResult<()> {
    let output = Command::new("feh")
        .args(args)
        .stdout(Stdio::null())
        .output()?;
    if !output.status.success() {
        bail!(
            "Command didn't execute successfully : {:#?}",
            String::from_utf8(output.stderr)?
        );
    }
    Ok(())
}
