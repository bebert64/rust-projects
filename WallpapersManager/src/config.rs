use firefox_sync_sdk::Client as FirefoxSyncClient;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Config {
    pub(crate) wallpapers_dir: String,
    pub(crate) single_screen_dir: String,
    pub(crate) dual_screen_dir: String,
    pub(crate) firefox_sync_client: FirefoxSyncClient,
}

config_helpers::config!("wallpapers_mgr");
