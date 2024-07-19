mod flickr;
mod wallhaven;
mod wallpaper_flare;

use crate::{wallpapers::sort, CONFIG};

use {don_error::*, url::Url};

pub fn download_all() -> DonResult<()> {
    let client = &CONFIG.firefox_sync_client;
    let to_download = client.get_folder("toolbar/Wallpaper/Download")?;
    for bookmark in to_download.bookmarks() {
        match Url::parse(&bookmark.url)?
            .domain()
            .ok_or_don_err("Can't get domain")?
        {
            "flickr" => flickr::download(&CONFIG.wallpapers_dir, &bookmark.url)?,
            "wallhaven" => wallhaven::download(&CONFIG.wallpapers_dir, &bookmark.url)?,
            "wallpaperflare" => wallpaper_flare::download(&CONFIG.wallpapers_dir, &bookmark.url)?,
            other_domain => bail!("Unsupported domain: {}", other_domain),
        }
    }

    sort::perform(false)?;

    Ok(())
}
