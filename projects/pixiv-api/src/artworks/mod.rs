#![allow(unused)]

use std::collections::BTreeMap;
use std::fmt::Formatter;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ArtworkRequest {
    pub word: String,
    pub order: String,
    pub mode: String,
    pub p: u32,
    pub csw: u32,
    pub s_mode: String,
    pub r#type: String,
    pub ratio: f32,
    pub ai_type: u32,
}

#[derive(Debug, Serialize)]
pub struct ArtworkTag {
    pub word: String,
    pub order: String,
    pub mode: String,
    pub csw: u32,
    pub s_mode: String,
    pub r#type: String,
    pub ratio: f32,
    pub allow_ai: bool,
}

impl ArtworkTag {
    pub fn build(&self, page: u32) -> ArtworkRequest {
        ArtworkRequest {
            word: self.word.clone(),
            order: self.order.clone(),
            mode: self.mode.clone(),
            p: page,
            csw: self.csw.clone(),
            s_mode: self.s_mode.clone(),
            r#type: self.r#type.clone(),
            ratio: self.ratio.clone(),
            ai_type: if self.allow_ai { 0 } else { 1 },
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

#[derive(Debug, Serialize, Default)]
pub struct IllustData {
    pub id: u32,
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
                    match id.parse::<u32>() {
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
    pub total: i64,
    #[serde(rename = "lastPage")]
    pub last_page: i64,
    #[serde(rename = "bookmarkRanges")]
    pub bookmark_ranges: Vec<Struct1>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtworksBody {
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

#[derive(Debug, Deserialize)]
pub struct PixivResponse<T> {
    pub error: bool,
    #[serde(default)]
    pub message: String,
    pub body: T,
}

