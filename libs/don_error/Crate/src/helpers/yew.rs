use crate::DonError;

#[derive(Clone, Copy)]
pub struct ReportedDonError {}

impl From<DonError> for ReportedDonError {
    fn from(err: DonError) -> Self {
        println!("{err:?}");
        ::web_sys::console::log_1(&format!("1").into());
        ReportedDonError {}
    }
}
