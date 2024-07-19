use {
    diesel::{
        connection::DefaultLoadingMode, query_dsl::methods::LoadQuery, result::Error as DieselError,
    },
    don_error::*,
    std::fmt,
};

pub trait GetOnlyResult<DB>: diesel::query_dsl::RunQueryDsl<DB> {
    /// If this is not run in a transaction, you will not be able to rollback in case your query
    /// updates several lines
    fn get_only_result<'conn, 'query: 'conn, U>(
        self,
        db: &'conn mut DB,
    ) -> Result<U, Fail<'conn, 'query, Self, DB, U>>
    where
        Self: LoadQuery<'query, DB, U> + 'conn,
        U: 'conn;
}

impl<DB, T: diesel::query_dsl::RunQueryDsl<DB>> GetOnlyResult<DB> for T {
    fn get_only_result<'conn, 'query: 'conn, U>(
        self,
        db: &'conn mut DB,
    ) -> Result<U, Fail<'conn, 'query, Self, DB, U>>
    where
        Self: LoadQuery<'query, DB, U> + 'conn,
        U: 'conn,
    {
        let mut results = self.load_iter::<U, DefaultLoadingMode>(db)?;
        match (results.next().transpose()?, results.next().transpose()?) {
            (None, _) => Err(Fail::<Self, DB, U>::NotFound),
            (Some(first), Some(second)) => {
                Err(Fail::MultipleInstancesReturned(PartiallyLoadedInstances {
                    first,
                    second,
                    others: results,
                }))
            }
            (Some(result), None) => Ok(result),
        }
    }
}

pub enum Fail<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
    DB: 'conn,
{
    GetFromPoolError(DonError),
    QueryError(DieselError),
    NotFound,
    MultipleInstancesReturned(PartiallyLoadedInstances<'conn, 'query, Q, DB, U>),
}

impl<'conn, 'query, Q, DB, U> fmt::Debug for Fail<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fail::GetFromPoolError(intl) => write!(f, "{intl:?}"),
            Fail::QueryError(derr) => write!(f, "{derr:?}"),
            Fail::NotFound => write!(f, "Fail::NotFound"),
            Fail::MultipleInstancesReturned { .. } => write!(f, "Fail::MultipleInstancesReturned"),
        }
    }
}

impl<'conn, 'query, Q, DB, U> From<Fail<'conn, 'query, Q, DB, U>> for DonError
where
    Q: LoadQuery<'query, DB, U>,
    DB: 'conn,
{
    fn from(fail: Fail<'conn, 'query, Q, DB, U>) -> Self {
        #[derive(Debug, thiserror::Error)]
        enum Failure {
            #[error("No instance returned on get_only_result")]
            NotFound,
            #[error("Multiple instances returned on get_only_result")]
            MultipleInstancesReturned,
        }

        match fail {
            Fail::GetFromPoolError(intl) => intl,
            Fail::QueryError(err) => err.into(),
            Fail::NotFound => Failure::NotFound.into(),
            Fail::MultipleInstancesReturned { .. } => Failure::MultipleInstancesReturned.into(),
        }
    }
}

pub struct PartiallyLoadedInstances<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
    DB: 'conn,
{
    pub first: U,
    pub second: U,
    pub others: Q::RowIter<'conn>,
}

impl<'conn, 'query, Q, DB, U> PartiallyLoadedInstances<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
{
    pub fn load_all(self) -> DonResult<Vec<U>> {
        let (_min, max) = self.others.size_hint();
        let mut results = Vec::with_capacity(max.unwrap_or(0) + 2);
        results.push(self.first);
        results.push(self.second);
        for instance in self.others {
            results.push(instance?);
        }
        Ok(results)
    }
}

pub enum UniqueFail<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
    DB: 'conn,
{
    NotFound,
    MultipleInstancesReturned(PartiallyLoadedInstances<'conn, 'query, Q, DB, U>),
}

impl<'conn, 'query, Q, DB, U> UniqueFail<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
{
    pub fn into_results(self) -> DonResult<Option<Vec<U>>> {
        Ok(match self {
            Self::NotFound => None,
            Self::MultipleInstancesReturned(partial) => Some(partial.load_all()?),
        })
    }
}

pub enum UniqueFailWithResults<U> {
    NotFound,
    MultipleInstancesReturned(Vec<U>),
}

impl<'conn, 'query, Q, DB, U> From<DieselError> for Fail<'conn, 'query, Q, DB, U>
where
    Q: LoadQuery<'query, DB, U>,
    DB: 'conn,
{
    fn from(e: DieselError) -> Self {
        Self::QueryError(e)
    }
}

pub trait GetOnlyResultExtension<'conn, 'query, Q, DB, T>
where
    Q: LoadQuery<'query, DB, T>,
    DB: 'conn,
{
    fn optional(self) -> Result<Option<T>, Fail<'conn, 'query, Q, DB, T>>;
    fn unique_or_fail(self) -> DonResult<Result<T, UniqueFail<'conn, 'query, Q, DB, T>>>;
    fn unique_or_fail_with_results(self) -> DonResult<Result<T, UniqueFailWithResults<T>>>
    where
        Self: Sized,
    {
        Ok(match self.unique_or_fail()? {
            Ok(val) => Ok(val),
            Err(UniqueFail::NotFound) => Err(UniqueFailWithResults::NotFound),
            Err(UniqueFail::MultipleInstancesReturned(partial)) => Err(
                UniqueFailWithResults::MultipleInstancesReturned(partial.load_all()?),
            ),
        })
    }
}
impl<'conn, 'query, Q, DB, T> GetOnlyResultExtension<'conn, 'query, Q, DB, T>
    for Result<T, Fail<'conn, 'query, Q, DB, T>>
where
    Q: LoadQuery<'query, DB, T>,
    DB: 'conn,
{
    fn optional(self) -> Result<Option<T>, Fail<'conn, 'query, Q, DB, T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Fail::NotFound) => Ok(None),
            Err(other) => Err(other),
        }
    }
    fn unique_or_fail(self) -> DonResult<Result<T, UniqueFail<'conn, 'query, Q, DB, T>>> {
        match self {
            Err(err @ (Fail::GetFromPoolError(_) | Fail::QueryError(_))) => Err(err.into()),
            Err(Fail::NotFound) => Ok(Err(UniqueFail::NotFound)),
            Err(Fail::MultipleInstancesReturned(partial)) => {
                Ok(Err(UniqueFail::MultipleInstancesReturned(partial)))
            }
            Ok(value) => Ok(Ok(value)),
        }
    }
}
