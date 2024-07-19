use super::{DUAL_SCREEN, SINGLE_SCREEN};

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

pub fn choose_once(mode: &Mode) -> DonResult<()> {
    use {Mode::*, ScreensConfig::*};
    match &screens_config()? {
        SingleScreen | DualScreenDifferentResolution | ThreeScreensOrMore => choose_single(),
        DualScreenSameResolution => match *mode {
            OnlySingle => choose_single(),
            OnlyDual => choose_dual(),
            FiftyFifty => choose_single_if_under_limit(0.5),
            ProportionateToNumberOfFiles => {
                // We take 2 single wallpapers at once, so we divide by 2 to get a
                // similar probability for each wallpaper
                let nb_single = PathBuf::from(&CONFIG.wallpapers_dir)
                    .join(SINGLE_SCREEN)
                    .read_dir()?
                    .count() as f64
                    / 2.;
                let nb_dual = PathBuf::from(&CONFIG.wallpapers_dir)
                    .join(DUAL_SCREEN)
                    .read_dir()?
                    .count() as f64;
                choose_single_if_under_limit(nb_single / (nb_single + nb_dual))
            }
        },
    }
}

pub fn choose_every_x_min(minutes: u64, mode: &Mode) -> DonResult<()> {
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
            try_or_report(|| choose_once(mode));
            sleep(Duration::new(minutes * 60, 0));
        },
        Err(_) => choose_once(mode),
    }
}

fn choose_single() -> DonResult<()> {
    run_command(
        Command::new("feh")
            .arg("--bg-max")
            .arg("--no-fehbg")
            .arg("--random")
            .arg(format!("{}/{SINGLE_SCREEN}", CONFIG.wallpapers_dir))
            .stdout(Stdio::null()),
    )
}

fn choose_dual() -> DonResult<()> {
    run_command(
        Command::new("feh")
            .arg("--bg-fill")
            .arg("--no-fehbg")
            .arg("--no-xinerama")
            .arg("--random")
            .arg(format!("{}/{DUAL_SCREEN}", CONFIG.wallpapers_dir))
            .stdout(Stdio::null()),
    )
}

fn choose_single_if_under_limit(limit: f64) -> DonResult<()> {
    let random_number = rand::thread_rng().gen::<f64>();
    if random_number < limit {
        choose_single()
    } else {
        choose_dual()
    }
}

fn run_command(command: &mut Command) -> DonResult<()> {
    let output = command.output()?;
    if !output.status.success() {
        bail!(
            "Command didn't execute successfully : {:#?}",
            String::from_utf8(output.stderr).expect("Valid error message")
        );
    }
    Ok(())
}
