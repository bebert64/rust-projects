use wallpapers_manager::{
    scrapers::download_all,
    wallpapers::{choose_every_x_min, choose_once, sort as sort_wallpapers, ChooseMode},
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
    Choose {
        #[arg(short, long, default_value = "proportionate-to-number-of-files")]
        mode: ChooseMode,
    },
    Scrap,
    Cron {
        #[arg(short = 'd', long)]
        minutes: u64,
        #[arg(short, long, default_value = "proportionate-to-number-of-files")]
        mode: ChooseMode,
    },
}

fn main() -> DonResult<()> {
    let wall_command = WallCommand::parse();
    match wall_command.command {
        Commands::Sort {
            force_sort_all_wallpapers,
        } => sort_wallpapers(force_sort_all_wallpapers)?,
        Commands::Choose { mode } => choose_once(&mode)?,
        Commands::Cron { minutes, mode } => choose_every_x_min(minutes, &mode)?,
        Commands::Scrap => download_all()?,
    }
    Ok(())
}
