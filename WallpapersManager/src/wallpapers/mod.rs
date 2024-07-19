pub mod choose;
pub mod sort;

pub use {
    choose::{choose_every_x_min, choose_once, Mode as ChooseMode},
    sort::perform as sort,
};

const SINGLE_SCREEN: &str = "Single_screen";
const DUAL_SCREEN: &str = "Dual_screen";
