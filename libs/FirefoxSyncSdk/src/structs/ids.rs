use {
    serde::{Deserialize, Serialize},
    std::ops::Deref,
};

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(from = "String")]
        pub struct $name {
            inner: String,
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.inner == other.inner
            }
        }

        impl Deref for $name {
            type Target = String;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<S: ToString> From<S> for $name {
            fn from(inner: S) -> Self {
                $name {
                    inner: inner.to_string(),
                }
            }
        }
    };
}

id!(BookmarkId);
id!(FolderId);
