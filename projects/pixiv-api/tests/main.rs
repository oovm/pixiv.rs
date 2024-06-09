use reqwest::Error;
use pixiv_api::artworks::{ArtworksRoot, ArtworkTag};

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

    let json: ArtworksRoot = response.json().await?;
    println!("{:#?}", json.body.illust.data[0]);

    Ok(())
}


