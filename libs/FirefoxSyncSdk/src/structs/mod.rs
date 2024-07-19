mod ids;

pub use ids::*;

use {
    serde::{Deserialize, Serialize},
    std::{
        collections::HashMap,
        ops::{Deref, DerefMut},
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum BookmarkOrFolder {
    Bookmark(Bookmark),
    Folder(Folder),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub title: String,
    #[serde(alias = "uri", alias = "bmkUri")]
    pub url: String,
    #[serde(rename = "parentid", default)]
    pub parent_id: Option<FolderId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: FolderId,
    pub title: String,
    #[serde(rename = "parentid", default)]
    pub parent_id: Option<FolderId>,
    pub(crate) children: Vec<Box<BookmarkOrFolder>>,
}

impl Folder {
    pub fn bookmarks(&self) -> impl Iterator<Item = &Bookmark> {
        self.children
            .iter()
            .filter_map(|child| match child.as_ref() {
                BookmarkOrFolder::Bookmark(bookmark) => Some(bookmark),
                BookmarkOrFolder::Folder(_folder) => None,
            })
    }

    pub fn sub_folders(&self) -> impl Iterator<Item = &Folder> {
        self.children
            .iter()
            .filter_map(|child| match child.as_ref() {
                BookmarkOrFolder::Folder(folder) => Some(folder),
                BookmarkOrFolder::Bookmark(_bookmark) => None,
            })
    }

    pub fn into_sub_folders(self) -> impl Iterator<Item = Folder> {
        self.children.into_iter().filter_map(|child| match *child {
            BookmarkOrFolder::Folder(folder) => Some(folder),
            BookmarkOrFolder::Bookmark(_bookmark) => None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkCollection {
    pub bookmarks: HashMap<String, Folder>,
}

impl Deref for BookmarkCollection {
    type Target = HashMap<String, Folder>;
    fn deref(&self) -> &Self::Target {
        &self.bookmarks
    }
}

impl DerefMut for BookmarkCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bookmarks
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deserialize_bookmark_with_uri() {
        let json = r#"{
            "type": "bookmark",
            "id": "1",
            "title": "Google",
            "uri": "https://www.google.com",
            "parentid": "0"
        }"#;
        let bookmark_or_folder: super::BookmarkOrFolder = serde_json::from_str(json).unwrap();
        let bookmark = match bookmark_or_folder {
            super::BookmarkOrFolder::Bookmark(bookmark) => bookmark,
            super::BookmarkOrFolder::Folder(_folder) => panic!("Expected a bookmark"),
        };
        assert_eq!(*bookmark.id, "1");
        assert_eq!(bookmark.title, "Google");
        assert_eq!(bookmark.url, "https://www.google.com");
        assert_eq!(*bookmark.parent_id.unwrap(), "0");
    }
    #[test]
    fn test_deserialize_bookmark_with_bmk_uri() {
        let json = r#"{
            "type": "bookmark",
            "id": "1",
            "title": "Google",
            "bmkUri": "https://www.google.com",
            "parentid": "0"
        }"#;
        let bookmark_or_folder: super::BookmarkOrFolder = serde_json::from_str(json).unwrap();
        let bookmark = match bookmark_or_folder {
            super::BookmarkOrFolder::Bookmark(bookmark) => bookmark,
            super::BookmarkOrFolder::Folder(_folder) => panic!("Expected a bookmark"),
        };
        assert_eq!(*bookmark.id, "1");
        assert_eq!(bookmark.title, "Google");
        assert_eq!(bookmark.url, "https://www.google.com");
        assert_eq!(*bookmark.parent_id.unwrap(), "0");
    }

    #[test]
    fn test_deserialize_folder() {
        let json = r#"{
        "type": "folder",
        "id": "1",
        "title": "Google",
        "children": []
    }"#;
        let bookmark_or_folder: super::BookmarkOrFolder = serde_json::from_str(json).unwrap();
        let folder = match bookmark_or_folder {
            super::BookmarkOrFolder::Bookmark(_bookmark) => panic!("Expected a folder"),
            super::BookmarkOrFolder::Folder(folder) => folder,
        };
        assert_eq!(*folder.id, "1");
        assert_eq!(folder.title, "Google");
        assert!(folder.parent_id.is_none());
    }

    #[test]
    fn test_deserialize_bookmarks_collection() {
        let json = include_str!("bookmarks.json");
        let collection: super::BookmarkCollection = serde_json::from_str(json).unwrap();
        assert!(collection.len() > 0);
        assert!(collection.get("menu").unwrap().children.len() > 0);
        assert_eq!(&collection.get("menu").unwrap().title, "menu");
    }
}
