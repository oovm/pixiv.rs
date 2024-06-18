#[derive(Copy, Clone, Debug, Default)]
pub enum PixivImageRatio {
    #[default]
    All = 0,
    Landscape,
    Portrait,
    Square,
}
#[derive(Copy, Clone, Debug, Default)]
pub enum PixiSafeMode {
    #[default]
    All = 0,
    Safe,
    R18,
    R18G,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum PixiMatchMode {
    #[default]
    ExactMatch = 0,
    PartialMatch,
    TitleOrCaption,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum PixiIllustrationType {
    #[default]
    All = 0,
    IllustrationAndIllustration,
    Illustration,
    Animation,
    Manga,
}


impl PixivImageRatio {
    pub fn encode_query(&self) -> Option<f32> {
        match self {
            Self::All => { None }
            Self::Landscape => { Some(0.5) }
            Self::Portrait => { Some(-0.5) }
            Self::Square => { Some(-0.5) }
        }
    }
}


impl PixiSafeMode {
    pub fn encode_query(&self) -> &'static str {
        match self {
            Self::All => { "all" }
            Self::Safe => { "safe" }
            Self::R18 => { "r18" }
            Self::R18G => { "r18" }
        }
    }
}



impl PixiIllustrationType {
    pub fn encode_query(&self) -> &'static str {
        match self {
            Self::All => { "all" }
            Self::IllustrationAndIllustration => { "illust_and_ugoira" }
            Self::Illustration => { "illust" }
            Self::Animation => { "ugoira" }
            Self::Manga => { "manga" }
        }
    }
    pub fn encode_route(&self) -> &'static str {
        match self {
            Self::All => { "artworks" }
            Self::IllustrationAndIllustration => { "illustrations" }
            Self::Illustration => { "illustrations" }
            Self::Animation => { "illustrations" }
            Self::Manga => { "manga" }
        }
    }
}


impl PixiMatchMode {
    pub fn encode_query(&self) -> &'static str {
        match self {
            Self::ExactMatch => { "s_tag_full" }
            Self::PartialMatch => { "s_tag" }
            Self::TitleOrCaption => { "s_tc" }
        }
    }
}