use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::path::PathBuf;
use rand::prelude::{IndexedRandom, SmallRng, ThreadRng};
use reqwest::{Client, Error, header};
use reqwest::header::{ACCEPT, AUTHORIZATION, COOKIE, HeaderValue, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use pixiv_api::artworks::{PixivResponse, ArtworkTag, ArtworksBody};

#[test]
fn ready() {
    println!("it works!")
}


#[tokio::test]
async fn main() -> Result<(), Error> {
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

    let json: PixivResponse<ArtworksBody> = response.json().await?;
    println!("{:#?}", json.body.illust.data);

    Ok(())
}


#[tokio::test]
async fn main2() -> Result<(), Error> {
    let url = "https://www.pixiv.net/ajax/illust/118456930/pages?lang=zh";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(include_str!("COOKIE.TXT")).unwrap(),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36",
        ),
    );
    let client = reqwest::Client::new();
    let response = client.get(url).headers(headers).send().await?;
    let json_data: PixivResponse<Vec<PixivImage>> = response.json().await?;

    println!("{:#?}", json_data);


    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct PixivImageUrls {
    pub thumb_mini: String,

    pub small: String,

    pub regular: String,

    pub original: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PixivImage {
    pub urls: PixivImageUrls,
    pub width: i64,
    pub height: i64,
}

pub struct PixivClient {
    rng: RefCell<ThreadRng>,
    root: PathBuf,
    agents: Vec<String>,
}

impl PixivClient {
    pub fn user_agent(&self) -> String {
        self.agents.choose(self.rng.borrow_mut().deref_mut()).unwrap().clone()
    }
}

impl PixivImageUrls {
    pub async fn download_original(&self, agent: &PixivClient) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.original.as_str();
        let client = Client::new();
        let mut response = client
            .get(url)
            .header(REFERER, "https://www.pixiv.net/")

            .send()
            .await?;

        let file_name = url.split('/').last().unwrap();
        let mut file = File::create(file_name).await?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes).await?;
        Ok(())
    }
}

#[tokio::test]
async fn main3() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://i.pximg.net/img-original/img/2024/05/14/18/23/09/118456930_p0.png";
    let referer = "https://www.pixiv.net/";

    let client = Client::new();
    let mut response = client
        .get(url)
        .header(REFERER, referer)
        // .header(USER_AGENT, user_agent)
        .send()
        .await?;

    let file_name = url.split('/').last().unwrap();
    let mut file = File::create(file_name).await?;
    let bytes = response.bytes().await?;
    file.write_all(&bytes).await?;

    println!("Image downloaded successfully!");


    Ok(())
}