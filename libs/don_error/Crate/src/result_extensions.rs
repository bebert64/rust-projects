use crate::*;

/// Utility functions for adding context to result
pub trait DonResultResultExtensions: Sized {
    type OK;
    type Err;
    fn err_ctx(self, context: &DonErrorContext) -> DonResult<Self::OK>;

    fn err_ctx_val<K, V>(self, key: K, value: V) -> DonResult<Self::OK>
    where
        DonErrorContextPair<K, V>: DonErrorContextPairT;

    fn err_ctx_ser<K, S: serde::Serialize>(self, key: K, serializable: S) -> DonResult<Self::OK>
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: DonErrorContextPairT;

    /// Helper function to divide an error between a fail part and an internal error part
    /// ```rust
    /// use internal_error::*;
    ///
    /// #[derive(thiserror::Error, Debug)]
    /// enum LibraryError {
    ///     #[error("Variant1")]
    ///     Variant1,
    ///     #[error("Variant2")]
    ///     Variant2,
    /// }
    ///
    /// fn library_fn(invalid_input: bool) -> Result<(), LibraryError> {
    ///     match invalid_input {
    ///         true => Err(LibraryError::Variant1),
    ///         false => Err(LibraryError::Variant2),
    ///     }
    /// }
    ///
    /// struct MyFail;
    ///
    /// assert!(matches!(
    ///     library_fn(true).map_err_to_fail(|e| match e {
    ///         LibraryError::Variant1 => Ok(MyFail),
    ///         _ => Err(e),
    ///     }),
    ///     Ok(Err(MyFail))
    /// ));
    ///
    /// assert!(library_fn(false)
    ///     .map_err_to_fail(|e| match e {
    ///         LibraryError::Variant1 => Ok(MyFail),
    ///         _ => Err(e),
    ///     })
    ///     .is_err());
    /// ```
    fn map_err_to_fail<F, E: Into<DonError>, Op: FnMut(Self::Err) -> Result<F, E>>(
        self,
        op: Op,
    ) -> DonResult<Result<Self::OK, F>>;
}

impl<T, E: Into<DonError>> DonResultResultExtensions for Result<T, E> {
    type OK = T;
    type Err = E;
    fn err_ctx(self, context: &DonErrorContext) -> DonResult<T> {
        self.map_err(|err| Into::<DonError>::into(err).with_ctx(context))
    }

    fn err_ctx_val<K, V>(self, key: K, value: V) -> DonResult<T>
    where
        DonErrorContextPair<K, V>: DonErrorContextPairT,
    {
        self.map_err(|err| Into::<DonError>::into(err).with_ctx_val(key, value))
    }

    fn err_ctx_ser<K, S: serde::Serialize>(self, key: K, serializable: S) -> DonResult<T>
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: DonErrorContextPairT,
    {
        self.err_ctx_val(key, DonErrorContextValueSerialize { serializable })
    }

    fn map_err_to_fail<F, E2: Into<DonError>, Op: FnMut(E) -> Result<F, E2>>(
        self,
        mut op: Op,
    ) -> DonResult<Result<Self::OK, F>> {
        match self {
            Ok(v) => Ok(Ok(v)),
            Err(e) => match op(e) {
                Ok(f) => Ok(Err(f)),
                Err(e) => Err(e.into()),
            },
        }
    }
}
