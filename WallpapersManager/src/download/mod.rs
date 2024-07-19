mod flickr;
mod wallhaven;
mod wallpaper_flare;

use crate::{wallpapers::sort, CONFIG};

use {
    don_error::*,
    firefox_sync_sdk::{Bookmark, Client as FirefoxSyncClient, CreateFolderInput},
    std::fs::File,
    url::Url,
};

pub fn perform() -> DonResult<()> {
    let client = &CONFIG.firefox_sync_client;
    let to_download = client.get_folder("toolbar/Wallpaper/Download")?;
    // TODO : Folder::get_or_create_subfolder
    let unsupported_domains_folder = match to_download
        .sub_folders()
        .find(|folder| folder.title == "Unsupported domains")
    {
        Some(folder) => folder,
        None => &client.create_folder(&CreateFolderInput {
            title: "Unsupported domains",
            parent_id: &to_download.id,
        })?,
    };
    fn download_and_delete_bookmark(
        client: &FirefoxSyncClient,
        download_fn: impl Fn(&str) -> DonResult<()>,
        bookmark: &Bookmark,
    ) -> DonResult<()> {
        download_fn(&bookmark.url)?;
        // TODO : Bookmark::delete
        client.delete_bookmark(&bookmark.id)
    }
    for bookmark in to_download.bookmarks() {
        {
            match Url::parse(&bookmark.url)?.domain() {
                Some("flickr") => download_and_delete_bookmark(client, flickr::download, bookmark)?,
                Some("wallhaven") => {
                    download_and_delete_bookmark(client, wallhaven::download, bookmark)?
                }
                Some("wallpaperflare") => {
                    download_and_delete_bookmark(client, wallpaper_flare::download, bookmark)?
                }
                // TODO Bookmark::move
                _ => client.move_bookmark(bookmark, unsupported_domains_folder)?,
            }
        }
    }

    sort::perform(false)?;

    Ok(())
}

fn download_file(link_to_file: &str) -> DonResult<()> {
    let mut wallpaper = File::create(format!(
        "{}/{}",
        CONFIG.wallpapers_dir,
        link_to_file
            .split('/')
            .next_back()
            .ok_or_don_err("split never returns empty iterator")?
    ))?;
    reqwest::blocking::get(link_to_file)?.copy_to(&mut wallpaper)?;

    Ok(())
}
