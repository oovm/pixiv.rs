use super::*;

#[derive(Clone, Debug)]
pub struct SearchIllustration {
    pub word: String,
    pub r#type: PixiIllustrationType,
    pub mode: PixiMatchMode,
    pub order: String,
    pub safe: PixiSafeMode,
    pub csw: u32,
    pub ratio: PixivImageRatio,
    pub min_width: Option<u32>,
    pub max_width: Option<u32>,
    pub min_height: Option<u32>,
    pub max_height: Option<u32>,
    pub allow_ai: bool,
    pub page: u32,
}


impl Serialize for SearchIllustration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut ser = serializer.serialize_struct("SearchTag", 8)?;
        ser.serialize_field("word", &self.word)?;
        ser.serialize_field("type", "all")?;
        ser.serialize_field("mode", self.safe.encode_query())?;
        ser.serialize_field("s_mode", self.mode.encode_query())?;

        if let Some(s) = self.ratio.encode_query() {
            ser.serialize_field("ratio", &s)?
        }
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

        ser.end()
    }
}


impl SearchIllustration {
    pub fn new(tag: &str) -> Self {
        Self {
            word: tag.to_string(),
            order: "data".to_string(),
            safe: PixiSafeMode::default(),
            csw: 1,
            r#type: PixiIllustrationType::default(),
            mode: PixiMatchMode::default(),
            page: 1,
            ratio: PixivImageRatio::default(),
            allow_ai: false,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: Some(9999),
        }
    }
    pub fn landscape(word: &str) -> Self {
        Self { ratio: PixivImageRatio::Landscape, min_width: Some(768), min_height: Some(512), ..Self::new(word) }
    }

    pub fn portrait(word: &str) -> Self {
        Self { ratio: PixivImageRatio::Portrait, min_width: Some(512), min_height: Some(768), ..Self::new(word) }
    }

    pub fn square(word: &str) -> Self {
        Self { min_width: Some(512), min_height: Some(512), ..Self::new(word) }
    }

    pub fn with_page(&self, page: u32) -> Self {
        Self { page, ..self.clone() }
    }
}

impl SearchIllustration {
    pub async fn request(&self, config: &PixivClient, auth: bool) -> Result<SearchTagPage, PixivError> {
        let request = Client::new()
            .get(format!("https://www.pixiv.net/ajax/search/illustrations/{0}", self.word))
            .query(&self);
        let response = if auth {
            request
                .header(USER_AGENT, config.user_agent())
                .header(COOKIE, &config.cookie)
                .send()
                .await?
        } else {
            request.send().await?
        };
        let json_data: PixivResponse<SearchTagPage> = response.json().await?;
        json_data.throw("SearchTag::request")
    }
}