use {
    don_error::{bail, DonResult},
    tetra::{
        window::{get_monitor_count, get_monitor_size},
        ContextBuilder,
    },
};

#[derive(Debug)]
pub(crate) enum ScreensConfig {
    SingleScreen,
    DualScreenSameResolution,
    DualScreenDifferentResolution,
    ThreeScreensOrMore,
}

pub(crate) fn screens_config() -> DonResult<ScreensConfig> {
    let context = ContextBuilder::new("Get monitors", 1, 1).build()?;
    match get_monitor_count(&context)? {
        0 => bail!("No screen availables"),
        1 => Ok(ScreensConfig::SingleScreen),
        2 if get_monitor_size(&context, 0).expect("Just checked there are two monitors")
            == get_monitor_size(&context, 1).expect("Just checked there are two monitors") =>
        {
            Ok(ScreensConfig::DualScreenSameResolution)
        }
        2 => Ok(ScreensConfig::DualScreenDifferentResolution),
        _ => Ok(ScreensConfig::ThreeScreensOrMore),
    }
}
