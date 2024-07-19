/// This is a structure to store a lazy representation of things that could be transformed into
/// owned data for being put into an error
#[derive(derive_deref::Deref, Default)]
pub struct DonErrorContext<'l> {
    pub(crate) inner: Vec<Box<dyn FromRefDonErrorContextPairT + 'l>>,
}

/// Represents the ability for a type to turned in an owned context pair from a reference
pub trait FromRefDonErrorContextPairT {
    fn to_owned(&self) -> (String, serde_json::Value);
}
// impl<K: std::fmt::Display, V> FromRefDonErrorContextPairT for DonErrorContextPair<K, V>
// where
//     for<'l> &'l V: Into<serde_json::Value>,
// {
//     fn to_owned(&self) -> (String, serde_json::Value) {
//         (self.key.to_string(), (&self.value).into())
//     }
// }
impl FromRefDonErrorContextPairT for Box<dyn FromRefDonErrorContextPairT + '_> {
    fn to_owned(&self) -> (String, serde_json::Value) {
        <dyn FromRefDonErrorContextPairT as FromRefDonErrorContextPairT>::to_owned(&**self)
    }
}

impl<'l> DonErrorContext<'l> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, pair: impl FromRefDonErrorContextPairT + 'l) -> &mut Self {
        self.inner.push(Box::new(pair));
        self
    }

    pub fn add_val<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        DonErrorContextPair<K, V>: FromRefDonErrorContextPairT + 'l,
    {
        self.add(DonErrorContextPair { key, value })
    }

    pub fn add_ser<K, S: serde::Serialize>(&mut self, key: K, serializable: S) -> &mut Self
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: FromRefDonErrorContextPairT + 'l,
    {
        self.add_val(key, DonErrorContextValueSerialize { serializable })
    }

    pub fn with(mut self, pair: impl FromRefDonErrorContextPairT + 'l) -> Self {
        self.inner.push(Box::new(pair));
        self
    }

    pub fn with_val<K, V>(mut self, key: K, value: V) -> Self
    where
        DonErrorContextPair<K, V>: FromRefDonErrorContextPairT + 'l,
    {
        self.add(DonErrorContextPair { key, value });
        self
    }

    pub fn with_ser<K, S: serde::Serialize>(mut self, key: K, serializable: S) -> Self
    where
        DonErrorContextPair<K, DonErrorContextValueSerialize<S>>: FromRefDonErrorContextPairT + 'l,
    {
        self.add_val(key, DonErrorContextValueSerialize { serializable });
        self
    }
}

impl std::fmt::Debug for DonErrorContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DonErrorContext");
        for var in &self.inner {
            let (name, value) = var.to_owned();
            match serde_json::to_string(&value) {
                Err(_) => s.field(&name, &value),
                Ok(mut value) => {
                    if value.len() > 100 {
                        value.truncate(100);
                        value += "...";
                    }
                    s.field(&name, &value)
                }
            };
        }
        s.finish()
    }
}

/// This is the typical structure that could be put in value: anything that is serializable can be
/// turned into a `serde_json::Value`
pub struct DonErrorContextValueSerialize<S: serde::Serialize> {
    pub serializable: S,
}

impl<S: serde::Serialize> From<&DonErrorContextValueSerialize<S>> for serde_json::Value {
    fn from(val: &DonErrorContextValueSerialize<S>) -> Self {
        serde_json::to_value(&val.serializable).unwrap_or_else(|err| {
            format!(
                "Failed to serialize {}: {}",
                core::any::type_name::<S>(),
                err
            )
            .into()
        })
    }
}

impl<S: serde::Serialize> From<DonErrorContextValueSerialize<S>> for serde_json::Value {
    fn from(val: DonErrorContextValueSerialize<S>) -> Self {
        (&val).into()
    }
}

/// Represents the ability for a type to be turned in an owned context pair
pub trait DonErrorContextPairT {
    fn into_owned(self) -> (String, serde_json::Value);
}

/// This structures is typically used to store a pair of key/value for context
pub struct DonErrorContextPair<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: Into<String>, V: Into<serde_json::Value>> DonErrorContextPairT
    for DonErrorContextPair<K, V>
{
    fn into_owned(self) -> (String, serde_json::Value) {
        (self.key.into(), self.value.into())
    }
}

impl<K: std::fmt::Display, V, T: std::ops::Deref<Target = DonErrorContextPair<K, V>>>
    DonErrorContextPairT for T
where
    for<'l> &'l V: Into<serde_json::Value>,
{
    fn into_owned(self) -> (String, serde_json::Value) {
        (self.key.to_string(), (&self.value).into())
    }
}
