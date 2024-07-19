pub use {
    config::{Config, Environment, File},
    directories::BaseDirs,
    don_error::*,
    lazy_static,
};

#[macro_export]
macro_rules! config {
    ($app:literal) => {
        $crate::lazy_static::lazy_static! {
            pub(crate) static ref CONFIG: Config = $crate::build_config($app).expect("Can't build config");
        }

        fn init() {
            $crate::lazy_static::initialize(&CONFIG);
        }
    };
}

pub fn build_config<'de, CONFIG: serde::Deserialize<'de>>(
    app_name: &'static str,
) -> DonResult<CONFIG> {
    let config_in_home_path = BaseDirs::new()
        .ok_or_don_err("Can't get base dirs")?
        .config_dir()
        .join(&format!("by_db/{}.toml", app_name));
    let config_in_working_dir_path = format!("{}.toml", app_name);
    let builder = Config::builder()
        .add_source(File::with_name(&config_in_home_path.to_string_lossy()).required(false))
        .add_source(File::with_name(&config_in_working_dir_path).required(false))
        .add_source(Environment::with_prefix(&app_name.to_uppercase()));
    let config_incomplete = builder.build_cloned().expect("Should be a valid config");
    Ok(builder
        .set_override("postgres.name", {
            if config_incomplete
                .get_string("mode")
                .unwrap_or_default()
                .to_lowercase()
                == *"prod"
            {
                config_incomplete
                    .get_string("postgres.name_prod")
                    .unwrap_or_default()
            } else {
                config_incomplete
                    .get_string("postgres.name_dev")
                    .unwrap_or_default()
            }
        })
        .expect("Key postgres.name should exist")
        .build()
        .expect("Should be a valid config")
        .try_deserialize()
        .unwrap_or_else(|err| {
            panic!(
                "Something went wrong while deserializing config. 
                Trying to read from {config_in_home_path:?} and {config_in_working_dir_path:?}.
                Err: {err:?}",
            )
        }))
}
