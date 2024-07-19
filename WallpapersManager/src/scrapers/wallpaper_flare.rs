use {
    don_error::{bail, err_msg, DonResult, DonResultOptionExtensions},
    std::fs::File,
};

pub(crate) fn download(wallpapers_dir: &str, mut url: &str) -> DonResult<()> {
    if !url.contains("www.wallpaperflare.com") {
        bail!("Invalid domain for Wallpaper Flare scraper")
    }
    for suffix in ["download", "download/"] {
        if url.ends_with(suffix) {
            url = url
                .strip_suffix(suffix)
                .ok_or_don_err("Just checked ends with suffix")?;
        }
    }
    println!("Downloading from {url}");

    let first_page = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&first_page);
    let selector = scraper::Selector::parse("a.link_btn.aq.mt20")
        .map_err(|_| err_msg!("Failed to create the selector for 'a.link_btn.aq.mt20'"))?;
    let link_to_download_page = document
        .select(&selector)
        .map(|elem| elem.value().attr("href"))
        .next()
        .ok_or_don_err("Couldn't find any tag 'a' with class 'link_btn aq mt20'")?
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'href'")?;

    let download_page = reqwest::blocking::get(link_to_download_page)?.text()?;
    let document = scraper::Html::parse_document(&download_page);
    let selector = scraper::Selector::parse("img#show_img")
        .map_err(|_| err_msg!("Failed to create the selector for 'img#show_img'"))?;
    let link_to_file = document
        .select(&selector)
        .map(|elem| elem.value().attr("src"))
        .next()
        .ok_or_don_err("Couldn't find any tag 'img' with id 'show_img'")?
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'src'")?;
    let split = &link_to_file.split('/').collect::<Vec<_>>();
    let filename = split.last().expect("url should contains '/'");

    let mut wallpaper = File::create(format!("{wallpapers_dir}/{filename}"))?;
    reqwest::blocking::get(link_to_file)?.copy_to(&mut wallpaper)?;

    Ok(())
}
