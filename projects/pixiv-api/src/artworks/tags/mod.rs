use super::*;

#[derive(Clone, Debug)]
pub struct SearchTag {
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

impl Serialize for SearchTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut ser = serializer.serialize_struct("SearchTag", 8)?;
        ser.serialize_field("word", &self.word)?;
        match self.ratio {
            PixivImageRatio::Landscape => { ser.serialize_field("ratio", &0.5)? }
            PixivImageRatio::Portrait => { ser.serialize_field("ratio", &-0.5)? }
            PixivImageRatio::Square => { ser.serialize_field("ratio", &-0.5)? }
            PixivImageRatio::All => {}
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
        ser.serialize_field("csw", &1);

        ser.end()
    }
}


impl SearchTag {
    pub fn new(word: &str) -> Self {
        Self {
            word: word.to_string(),
            order: "data".to_string(),
            mode: "all".to_string(),
            csw: 1,
            s_mode: "s_tag".to_string(),
            r#type: "illust".to_string(),
            page: 1,
            ratio: PixivImageRatio::All,
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

impl SearchTag {
    pub async fn request(&self, config: &PixivClient, auth: bool) -> Result<SearchTagPage, PixivError> {
        let request = Client::new()
            .get(format!("https://www.pixiv.net/ajax/search/illustrations/{0}", self.word))
            .query(&self);
        let response = if auth {
            request
                .header(COOKIE, &config.cookie)
                .header(USER_AGENT, config.user_agent())
                .send()
                .await?
        } else {
            request.send().await?
        };
        let json_data: PixivResponse<SearchTagPage> = response.json().await?;
        json_data.throw("")
    }
}