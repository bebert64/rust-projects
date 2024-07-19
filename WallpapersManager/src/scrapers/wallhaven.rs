use {
    don_error::{bail, err_msg, DonResult, DonResultOptionExtensions},
    std::fs::File,
};

pub(crate) fn download(wallpapers_dir: &str, url: &str) -> DonResult<()> {
    if !url.contains("wallhaven.cc") {
        bail!("Invalid domain for Wallhaven scraper")
    }
    println!("Downloading from {url}");

    let download_page = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&download_page);
    let selector = scraper::Selector::parse("img#wallpaper")
        .map_err(|_| err_msg!("Failed to create the selector for 'img#wallpaper'"))?;
    let link_to_file = document
        .select(&selector)
        .map(|elem| elem.value().attr("data-cfsrc"))
        .next()
        .ok_or_don_err("Couldn't find any tag 'img' with id 'wallpaper'")?
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'data-cfsrc'")?;
    let split = &link_to_file.split('/').collect::<Vec<_>>();
    let filename = split.last().expect("url should contains '/'");

    let mut wallpaper = File::create(format!("{wallpapers_dir}/{filename}"))?;
    reqwest::blocking::get(link_to_file)?.copy_to(&mut wallpaper)?;

    Ok(())
}
