use don_error::*;

pub(crate) fn download(url: &str) -> DonResult<()> {
    println!("Downloading from {url}");

    let source_code = reqwest::blocking::get(format!("{url}/sizes/o"))?.text()?;
    let html = scraper::Html::parse_document(&source_code);
    let selector_str = "div#allsizes-photo>img";
    let err_ctx = DonErrorContext::new().with_ser("source code", &source_code);

    super::download_file(
        html.select(
            &scraper::Selector::parse(selector_str)
                .map_err(|_| err_msg!("Failed to create the selector for '{selector_str}'"))?,
        )
        .next()
        .ok_or_don_err("Couldn't find any tag 'div' with id 'allsizes-photo'")
        .err_ctx(&err_ctx)?
        .value()
        .attr("src")
        .ok_or_don_err("The tag was found but doesn't contain an attribute 'src'")
        .err_ctx(&err_ctx)?,
    )
}
