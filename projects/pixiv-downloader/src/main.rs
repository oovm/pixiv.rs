use std::cell::RefCell;
use std::path::{Path};
use pixiv_api::artworks::{ ArtworkTag, SearchPage, PixivArtwork, PixivClient};
use pixiv_api::{PixivError};

#[tokio::main]
async fn main() -> Result<(), PixivError> {
    let config = PixivClient {
        rng: RefCell::new(Default::default()),
        root: Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
        agents: vec![
            UA.to_string()
        ],
        cookie: include_str!("COOKIE.TXT").to_string(),
        wait: 1.0..2.0,
    };

    let params = ArtworkTag::portrait("%E7%A3%94");
    let json: SearchPage = params.request(&config).await?;
    for data in json.illust.data.clone() {
        let art = PixivArtwork { id: data.id };
        if art.id == 0 {
            // Ads pictures
            continue;
        }
        match art.download_original(&config).await {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    for page in 2..=json.illust.last_page {
        let json = params.with_page(page).request(&config).await?;
        for data in json.illust.data {
            let art = PixivArtwork { id: data.id };
            if art.id == 0 {
                // Ads pictures
                continue;
            }
            match art.download_original(&config).await {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }


    Ok(())
}

const UA: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36";
