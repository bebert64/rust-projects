mod bookmarks;
mod client;
mod structs;

pub use {
    bookmarks::{CreateBookmarkInput, CreateFolderInput},
    client::Client,
    structs::*,
};
