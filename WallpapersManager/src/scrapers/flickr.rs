use {
    don_error::{bail, err_msg, DonResult, DonResultOptionExtensions},
    std::fs::File,
};

pub(crate) fn download(wallpapers_dir: &str, url: &str) -> DonResult<()> {
    if !url.contains("www.flickr.com") {
        bail!("Invalid domain for Flickr scraper")
    }
    println!("Downloading from {url}");

    let download_page = reqwest::blocking::get(format!("{url}/sizes/o"))?.text()?;
    let document = scraper::Html::parse_document(&download_page);
    let selector = scraper::Selector::parse("div#allsizes-photo>img")
        .map_err(|_| err_msg!("Failed to create the selector for 'div#allsizes-photo>img'"))?;
    let link_to_file = document
        .select(&selector)
        .map(|elem| elem.value().attr("src"))
        .next()
        .ok_or_don_err("Couldn't find any tag 'div' with id 'allsizes-photo'")?
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'src'")?;
    let split = &link_to_file.split('/').collect::<Vec<_>>();
    let filename = split.last().expect("url should contains '/'");

    let mut wallpaper = File::create(format!("{wallpapers_dir}/{filename}"))?;
    reqwest::blocking::get(link_to_file)?.copy_to(&mut wallpaper)?;

    Ok(())
}
