use crate::{client::Client, structs::*};

use don_error::*;

#[derive(Debug, serde::Serialize)]
pub struct CreateBookmarkInput<'l> {
    pub url: &'l str,
    pub title: &'l str,
    pub parent_id: &'l FolderId,
}

impl Client {
    pub fn get_all_bookmarks(&self) -> DonResult<BookmarkCollection> {
        self.try_command_and(&LIST_BOOKMARKS_ARGS, |output| {
            serde_json::from_str(&output).map_err(Into::into)
        })
    }

    pub fn get_folder(&self, path: &str) -> DonResult<Folder> {
        let mut collection = self.get_all_bookmarks()?;
        if path.is_empty() {
            bail!("Empty path")
        };
        let mut path_split = path.split('/');
        let mut folder = collection
            .remove(path_split.next().ok_or_don_err("Just checked not empty")?)
            .ok_or_don_err("{path} doesn't exists in collection")?;
        for folder_name in path_split {
            folder = folder
                .into_sub_folders()
                .find(|folder| folder.title == folder_name)
                .ok_or_don_err(format!("{folder_name} doesn't exists"))?;
        }
        Ok(folder)
    }

    pub fn create_bookmark(&self, bookmark: &CreateBookmarkInput) -> DonResult<()> {
        self.try_command(&[
            "bookmarks",
            "create",
            "bookmark",
            bookmark.title,
            bookmark.url,
            "--parent",
            bookmark.parent_id,
        ])
    }

    pub fn delete_bookmark(&self, bookmark_id: &BookmarkId) -> DonResult<()> {
        self.try_command(&["bookmarks", "delete", bookmark_id])
    }

    pub fn move_bookmark(&self, bookmark: &Bookmark, parent: &Folder) -> DonResult<()> {
        if bookmark.parent_id.as_ref() == Some(&parent.id) {
            return Ok(());
        };
        self.create_bookmark(&CreateBookmarkInput {
            url: &bookmark.url,
            title: &bookmark.title,
            parent_id: &parent.id,
        })?;
        self.delete_bookmark(&bookmark.id)
    }
}

pub(crate) const LIST_BOOKMARKS_ARGS: [&str; 5] = [
    "bookmarks",
    "list",
    "--format",
    "json",
    "--ignore-schema-errors",
];
