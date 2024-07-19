pub use {
    config::{Config, Environment, File},
    homedir::get_my_home,
    lazy_static::lazy_static,
};

#[macro_export]
macro_rules! config {
    ($app:literal) => {
        $crate::lazy_static! {
            pub(crate) static ref CONFIG: Config = $crate::build_config($app);
        }
    };
}

pub fn build_config<'de, CONFIG: serde::Deserialize<'de>>(app_name: &'static str) -> CONFIG {
    let builder = Config::builder()
        .add_source(
            File::with_name(
                &get_my_home()
                    .ok()
                    .flatten()
                    .map(|home_dir| {
                        home_dir
                            .join(&format!(".config/by_db/{}.toml", app_name))
                            .to_str()
                            .expect("Should be a valid path")
                            .to_owned()
                    })
                    .unwrap_or_default(),
            )
            .required(false),
        )
        .add_source(File::with_name(&format!("{}.toml", app_name)).required(false))
        .add_source(Environment::with_prefix(&app_name.to_uppercase()));
    let config_incomplete = builder.build_cloned().expect("Should be a valid config");
    builder
        .set_override("postgres.name", {
            if config_incomplete.get_string("mode").unwrap_or_default().to_lowercase() == *"prod" {
                config_incomplete.get_string("postgres.name_prod").unwrap_or_default()
            } else {
                config_incomplete.get_string("postgres.name_dev").unwrap_or_default()
            }
        })
        .expect("Key postgres.name should exist")
        .build()
        .expect("Should be a valid config")
        .try_deserialize()
        .unwrap_or_else(|err|
            panic!(
                "Something went wrong while deserializing config. Trying to read from {:?} and {:?}. Err: {err:?}",
                get_my_home()
                    .ok()
                    .flatten()
                    .map(|home_dir| home_dir.join(&format!(".config/by_db/{}.toml", app_name))
                    .to_str()
                    .expect("Should be a valid path")
                    .to_owned()),
                File::with_name(&format!("{}.toml", app_name))
            )
        )
}
