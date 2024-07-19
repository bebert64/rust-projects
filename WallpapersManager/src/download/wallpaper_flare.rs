use don_error::*;

pub(crate) fn download(mut url: &str) -> DonResult<()> {
    for suffix in ["download", "download/"] {
        url = url.strip_suffix(suffix).unwrap_or(url);
    }
    println!("Downloading from {url}");

    let first_source_code = reqwest::blocking::get(url)?.text()?;
    let first_html = scraper::Html::parse_document(&first_source_code);
    let selector_str = "a.link_btn.aq.mt20";
    let err_ctx = DonErrorContext::new().with_ser("source code", &first_source_code);

    let link_to_download_page = first_html
        .select(
            &scraper::Selector::parse(selector_str)
                .map_err(|_| err_msg!("Failed to create the selector for '{selector_str}'"))?,
        )
        .next()
        .ok_or_don_err("Couldn't find any tag 'a' with class 'link_btn aq mt20'")
        .err_ctx(&err_ctx)?
        .value()
        .attr("href")
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'href'")
        .err_ctx(&err_ctx)?;

    let source_code = reqwest::blocking::get(link_to_download_page)?.text()?;
    let html = scraper::Html::parse_document(&source_code);
    let selector_str = "a.link_btn.aq.mt20";
    let err_ctx = DonErrorContext::new().with_ser("source code", &source_code);

    super::download_file(
        html.select(
            &scraper::Selector::parse(selector_str)
                .map_err(|_| err_msg!("Failed to create the selector for '{selector_str}'"))?,
        )
        .next()
        .ok_or_don_err("Couldn't find any tag 'img' with id 'show_img'")
        .err_ctx(&err_ctx)?
        .value()
        .attr("src")
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'src'")
        .err_ctx(&err_ctx)?,
    )
}
