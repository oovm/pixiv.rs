use std::cell::RefCell;
use std::path::{Path, PathBuf};
use rand::Rng;
use reqwest::{Client, Error, header};
use pixiv_api::artworks::{PixivResponse, ArtworkTag, SearchPage, PixivArtwork, PixivClient};
use pixiv_api::{PixivError, PixivImage};

#[test]
fn ready() {
    println!("it works!")
}


#[tokio::test]
async fn main() -> Result<(), PixivError> {
    let config = PixivClient {
        rng: RefCell::new(Default::default()),
        root: Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
        agents: vec![
            UA.to_string()
        ],
        cookie: include_str!("COOKIE.TXT").to_string(),
        wait: 0.5..1.5,
    };


    let url = "https://www.pixiv.net/ajax/search/illustrations/%E7%A3%94";

    let params = ArtworkTag {
        word: "%E7%A3%94".to_string(),
        order: "date".to_string(),
        mode: "all".to_string(),
        csw: 1,
        s_mode: "s_tag".to_string(),
        r#type: "illust".to_string(),
        ratio: -0.5,
        allow_ai: false,
    };

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .query(&params.build(1))
        .send()
        .await?;

    let json: PixivResponse<SearchPage> = response.json().await?;

    for data in json.body.illust.data {
        let art = PixivArtwork { id: data.id };
        art.download_original(&config).await?;
    }


    Ok(())
}

const UA: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36";
