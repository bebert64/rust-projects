use wallpapers_manager::{
    change_wallpaper_every_n_minutes, change_wallpaper_once, download_wallpapers, sort_wallpapers,
    ChangeMode,
};

use {
    clap::{Parser, Subcommand},
    don_error::*,
};

#[derive(Parser)]
struct WallCommand {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sort {
        #[arg(short, long, default_value = "false")]
        force_sort_all_wallpapers: bool,
    },
    Change {
        #[arg(short, long, default_value = "proportionate-to-number-of-files")]
        mode: ChangeMode,
    },
    Download,
    Cron {
        #[arg(short = 'd', long)]
        minutes: u64,
        #[arg(short, long, default_value = "proportionate-to-number-of-files")]
        mode: ChangeMode,
    },
}

fn main() -> DonResult<()> {
    let wall_command = WallCommand::parse();
    match wall_command.command {
        Commands::Sort {
            force_sort_all_wallpapers,
        } => sort_wallpapers(force_sort_all_wallpapers)?,
        Commands::Change { mode } => change_wallpaper_once(&mode)?,
        Commands::Cron { minutes, mode } => change_wallpaper_every_n_minutes(minutes, &mode)?,
        Commands::Download => download_wallpapers()?,
    }
    Ok(())
}
