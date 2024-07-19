use crate::*;

pub trait DonResultOptionExtensions: Sized {
    type SOME;

    fn ok_or_don_err(self, msg: impl Into<String>) -> DonResult<Self::SOME>;
}

impl<T> DonResultOptionExtensions for Option<T> {
    type SOME = T;

    fn ok_or_don_err(self, msg: impl Into<String>) -> DonResult<T> {
        self.ok_or_else(|| err_msg!(msg.into()))
    }
}
