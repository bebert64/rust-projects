use crate::*;

use std::collections::BTreeMap;

/// Wrapper around `anyhow::Error` with support for additional json context
#[must_use = "`DonError`s should almost always be `return`ed or `.report()`ed."]
pub struct DonError {
    inner: Box<DonErrorInner>,
}

#[allow(clippy::manual_non_exhaustive)]
pub struct DonErrorInner {
    pub err: anyhow::Error,
    pub context: BTreeMap<String, serde_json::Value>,
    _private: (),
}

impl DonError {
    pub fn report(&self) {
        println!("DonError : {self:?}")
    }

    pub fn with_ctx(mut self, context: &DonErrorContext) -> Self {
        self.add_ctx(context);
        self
    }

    pub fn with_ctx_val<K, V>(mut self, key: K, value: V) -> Self
    where
        DonErrorContextPair<K, V>: DonErrorContextPairT,
    {
        self.add_ctx_val(key, value);
        self
    }

    pub fn with_ctx_ser<K, S: serde::Serialize>(self, key: K, serializable: S) -> Self
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: DonErrorContextPairT,
    {
        self.with_ctx_val(key, DonErrorContextValueSerialize { serializable })
    }

    pub fn add_ctx(&mut self, ctx: &DonErrorContext) -> &mut Self {
        ctx.iter().for_each(|ctx_pair| {
            let (key, value) = FromRefDonErrorContextPairT::to_owned(&**ctx_pair);
            self.add_ctx_pair_inner(key, value)
        });
        self
    }

    pub fn add_ctx_val<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        DonErrorContextPair<K, V>: DonErrorContextPairT,
    {
        self.add_ctx_pair(DonErrorContextPair { key, value });
        self
    }

    pub fn add_ctx_ser<K, S: serde::Serialize>(&mut self, key: K, serializable: S) -> &mut Self
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: DonErrorContextPairT,
    {
        self.add_ctx_val(key, DonErrorContextValueSerialize { serializable })
    }

    pub fn add_ctx_pair(&mut self, ctx_pair: impl DonErrorContextPairT) {
        let (key, value) = ctx_pair.into_owned();
        self.add_ctx_pair_inner(key, value);
    }

    #[allow(clippy::map_entry)] // Using Entry would require to do one more clone of key
    fn add_ctx_pair_inner(&mut self, key: String, value: serde_json::Value) {
        if self.context.contains_key(&key) {
            use std::collections::btree_map::Entry;
            for i in 1.. {
                match self.context.entry(format!("{}_{}", key, i)) {
                    Entry::Vacant(vacant) => {
                        vacant.insert(value);
                        break;
                    }
                    Entry::Occupied(_) => {}
                }
            }
        } else {
            self.context.insert(key, value);
        }
    }

    pub fn into_inner(self) -> (anyhow::Error, BTreeMap<String, serde_json::Value>) {
        let inner = *(self.inner);
        (inner.err, inner.context)
    }
}

impl<T: Into<anyhow::Error>> From<T> for DonError {
    fn from(err: T) -> Self {
        Self {
            inner: Box::new(DonErrorInner {
                err: err.into(),
                context: std::collections::BTreeMap::new(),
                _private: (),
            }),
        }
    }
}

impl std::fmt::Display for DonError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.err, fmt)
    }
}

impl std::fmt::Debug for DonError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DonError")
            .field("err", &self.err)
            .field("context", &self.context)
            .finish()
    }
}

impl std::ops::Deref for DonError {
    type Target = DonErrorInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DonError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl std::ops::Deref for DonErrorInner {
    type Target = anyhow::Error;
    fn deref(&self) -> &Self::Target {
        &self.err
    }
}
impl std::convert::AsRef<anyhow::Error> for DonError {
    fn as_ref(&self) -> &anyhow::Error {
        &self.err
    }
}
impl AsRef<dyn std::error::Error + Send + Sync + 'static> for DonError {
    fn as_ref(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        self.err.as_ref()
    }
}
