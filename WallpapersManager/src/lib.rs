mod config;
mod download;
mod monitors;
mod wallpapers;

pub use {
    download::perform as download_wallpapers,
    wallpapers::{
        change::{
            every_n_min as change_wallpaper_every_n_minutes, once as change_wallpaper_once,
            Mode as ChangeMode,
        },
        sort::perform as sort_wallpapers,
    },
};

pub(crate) use config::CONFIG;
