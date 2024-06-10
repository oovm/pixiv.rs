#![allow(unused)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::ops::{DerefMut, Range};
use std::path::{Path, PathBuf};
use std::time::Duration;
use rand::prelude::{IndexedRandom, ThreadRng};
use rand::Rng;
use reqwest::Client;
use reqwest::header::{COOKIE, REFERER, USER_AGENT};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde_json::Value;
use tokio::fs::File;
use tokio::time::Sleep;
use crate::{PixivError, PixivImage};

pub mod images;

#[derive(Debug, Deserialize)]
pub struct PixivResponse<T> {
    pub error: bool,
    #[serde(default)]
    pub message: String,
    pub body: T,
}

#[derive(Debug)]
pub struct ArtworkRequest {
    pub word: String,
    pub order: String,
    pub mode: String,
    pub p: u64,
    pub csw: u32,
    pub s_mode: String,
    pub r#type: String,
}

impl Serialize for ArtworkTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut ser = serializer.serialize_struct("ArtworkTag", 8)?;
        ser.serialize_field("word", &self.word)?;


        match self.ratio { _ => { ser.serialize_field("ratio", &-0.5)? } }

        if let Some(s) = self.min_width {
            ser.serialize_field("wlt", &s)?
        }
        if let Some(s) = self.max_width {
            ser.serialize_field("wgt", &s)?
        }
        if let Some(s) = self.min_height {
            ser.serialize_field("hlt", &s)?
        }
        if let Some(s) = self.max_height {
            ser.serialize_field("hgt", &s)?
        }


        if !self.allow_ai {
            ser.serialize_field("ai_type", &1)?
        }
        ser.serialize_field("csw", &1);


        ser.end()
    }
}

#[derive(Debug)]
pub enum PixivImageRatio {
    Landscape,
    Port,
    Square,
    All,
}


#[derive(Debug)]
pub struct ArtworkTag {
    pub word: String,
    pub order: String,
    pub mode: String,
    pub csw: u32,
    pub s_mode: String,
    pub r#type: String,
    pub page: u32,
    pub ratio: PixivImageRatio,
    pub allow_ai: bool,
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,

}

impl ArtworkTag {
    pub fn new(word: &str, page: u32) -> Self {
        Self {
            word: word.to_string(),
            order: "data".to_string(),
            mode: "all".to_string(),
            csw: 1,
            s_mode: "s_tag".to_string(),
            r#type: "illust".to_string(),
            page,
            ratio: PixivImageRatio::All,
            allow_ai: false,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: Some(9999),
        }
    }
    pub fn landscape(word: &str, page: u32) -> Self {
        Self {
            ratio: PixivImageRatio::Landscape,
            min_width: Some(768),
            min_height: Some(512),
            ..Self::new(word, page)
        }
    }

    pub fn potial(word: &str, page: u32) -> Self {
        Self {
            ratio: PixivImageRatio::Port,
            min_width: Some(512),
            min_height: Some(768),
            ..Self::new(word, page)
        }
    }

    pub fn square(word: &str, page: u32) -> Self {
        Self {
            min_width: Some(512),
            min_height: Some(512),
            ..Self::new(word, page)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternateLanguages {
    pub ja: String,
    pub en: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub title: String,
    pub description: String,
    pub canonical: String,
    #[serde(rename = "alternateLanguages")]
    pub alternate_languages: AlternateLanguages,
    #[serde(rename = "descriptionHeader")]
    pub description_header: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraData {
    pub meta: Meta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct3 {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneConfig {
    pub logo: Struct3,
    pub header: Struct3,
    pub footer: Struct3,
    pub infeed: Struct3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct2 {
    pub zh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Popular {
    pub recent: Value,
    pub permanent: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct1 {
    pub min: Option<i64>,
    pub max: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TitleCaptionTranslation {
    #[serde(rename = "workTitle")]
    pub work_title: Value,
    #[serde(rename = "workCaption")]
    pub work_caption: Value,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct IllustData {
    pub id: u64,
    pub tags: Vec<String>,
    pub title: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    #[serde(flatten)]
    pub unknown_fields: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for IllustData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let mut data = IllustData::default();
        let visitor = IllustDataVisitor { data: &mut data };
        deserializer.deserialize_map(visitor)?;
        Ok(data)
    }
}


struct IllustDataVisitor<'i> {
    data: &'i mut IllustData,
}

impl<'i, 'de> Visitor<'de> for IllustDataVisitor<'i> {
    type Value = ();

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "id" => {
                    let id = map.next_value::<String>()?;
                    match id.parse() {
                        Ok(id) => {
                            self.data.id = id;
                        }
                        Err(..) => {}
                    }
                }
                "tags" => {
                    self.data.tags = map.next_value()?
                }
                "title" => {
                    self.data.title = map.next_value()?
                }

                "description" => {
                    self.data.description = map.next_value()?
                }
                "width" => {
                    self.data.width = map.next_value()?;
                }
                "height" => {
                    self.data.height = map.next_value()?;
                }

                unknown => {
                    let value = map.next_value::<Value>()?;

                    self.data.unknown_fields.insert(key, value);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Illust {
    pub data: Vec<IllustData>,
    pub total: u64,
    #[serde(rename = "lastPage")]
    pub last_page: u64,
    #[serde(rename = "bookmarkRanges")]
    pub bookmark_ranges: Vec<Struct1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPage {
    #[serde(rename = "illust")]
    pub illust: Illust,
    pub popular: Popular,
    #[serde(rename = "relatedTags")]
    pub related_tags: Vec<String>,
    #[serde(rename = "tagTranslation")]
    pub tag_translation: Value,
    #[serde(rename = "zoneConfig")]
    pub zone_config: ZoneConfig,
    #[serde(rename = "extraData")]
    pub extra_data: ExtraData,
}


pub struct PixivClient {
    pub rng: RefCell<ThreadRng>,
    pub root: PathBuf,
    pub agents: Vec<String>,
    pub cookie: String,
    pub wait: Range<f32>,
}

impl PixivClient {
    pub fn user_agent(&self) -> &str {
        self.agents.choose(self.rng.borrow_mut().deref_mut()).unwrap()
    }
    pub fn cooldown(&self) -> Sleep {
        let time = self.rng.borrow_mut().gen_range(self.wait.clone());
        tokio::time::sleep(Duration::from_secs_f32(time))
    }
}

pub struct PixivArtwork {
    pub id: u64,
}

impl PixivArtwork {
    pub async fn download_original(&self, client: &PixivClient) -> Result<usize, PixivError> {
        if self.get_skip_mark(&client.root) {
            println!("Skip downloading artwork {}", self.id);
            return Ok(0);
        }
        let data = self.get_image_urls(&client.cookie, client.user_agent()).await?;
        // WAIT!!! error 429 here!
        client.cooldown().await;
        // for image in data.iter() {
        //     image.download_original(&client.root).await?;
        // }
        let download_tasks = data.iter().cloned().map(|image| {
            let path = client.root.clone();
            tokio::task::spawn(async move {
                image.download_original(&path).await
            })
        }).collect::<Vec<_>>();
        let tasks = futures::future::join_all(download_tasks).await;
        for task in tasks {
            match task {
                Ok(o) => {
                    match o {
                        Ok(_) => {}
                        Err(e) => {
                            println!("DownloadError: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("JoinError: {}", e);
                }
            }
        }
        self.set_skip_mark(&client.root).await?;
        println!("Downloaded artwork {}", self.id);
        Ok(data.len())
    }

    pub async fn get_image_urls(&self, cookie: &str, agent: &str) -> Result<Vec<PixivImage>, PixivError> {
        let url = format!("https://www.pixiv.net/ajax/illust/{0}/pages?lang=zh", self.id);
        let client = Client::new();
        let response = client.get(url).header(USER_AGENT, agent).header(COOKIE, cookie).send().await?;
        let json_data: PixivResponse<Vec<PixivImage>> = response.json().await?;
        match json_data.error {
            true => {
                Err(PixivError::request_error(json_data.message, format!("PixivArtwork::get_image_urls({})", self.id)))
            }
            false => {
                Ok(json_data.body)
            }
        }
    }
    pub fn get_skip_mark(&self, folder: &Path) -> bool {
        let path = folder.join("skip").join(self.id.to_string());
        path.exists()
    }
    pub async fn set_skip_mark(&self, folder: &Path) -> Result<PathBuf, PixivError> {
        let path = folder.join("skip");
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        File::create(path.join(self.id.to_string())).await?;
        Ok(path)
    }
}
