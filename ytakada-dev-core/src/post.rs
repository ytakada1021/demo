use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::AssertError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id: PostId,
    title: PostTitle,
    content: PostContent,
    tags: Vec<Tag>,
    posted_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>,
}

impl Post {
    pub fn from_markdown(id: PostId, markdown: &impl Into<String>) -> Self {
        Self {
            id,
            title: PostTitle::new("Sample Title").unwrap(),
            content: PostContent::new("<h1>This is sample.</h1>").unwrap(),
            tags: vec![Tag::new("tag1").unwrap(), Tag::new("tag2").unwrap()],
            posted_at: Local::now(),
            updated_at: None,
        }
    }

    pub fn id(&self) -> &PostId {
        &self.id
    }

    pub fn title(&self) -> &PostTitle {
        &self.title
    }

    pub fn content(&self) -> &PostContent {
        &self.content
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn posted_at(&self) -> &DateTime<Local> {
        &self.posted_at
    }

    pub fn updated_at(&self) -> &Option<DateTime<Local>> {
        &self.updated_at
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostId(String);

impl PostId {
    pub fn new(value: impl Into<String>) -> Result<Self, AssertError> {
        let value: String = value.into();
        let len = value.len();

        if len >= 1 && len <= 100 {
            Ok(PostId(value))
        } else {
            Err(AssertError)
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostTitle(String);

impl PostTitle {
    pub fn new(value: impl Into<String>) -> Result<Self, AssertError> {
        let value: String = value.into();
        let len = value.len();

        if len >= 1 && len <= 200 {
            Ok(PostTitle(value))
        } else {
            Err(AssertError)
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(value: impl Into<String>) -> Result<Self, AssertError> {
        let value: String = value.into();
        let len = value.len();

        if len >= 1 && len <= 50000 {
            Ok(PostContent(value))
        } else {
            Err(AssertError)
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag(String);

impl Tag {
    pub fn new(value: impl Into<String>) -> Result<Self, AssertError> {
        let value: String = value.into();
        let len = value.len();

        if len >= 1 && len <= 30 {
            Ok(Tag(value))
        } else {
            Err(AssertError)
        }
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}
