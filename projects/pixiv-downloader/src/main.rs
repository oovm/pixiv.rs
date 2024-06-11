use std::path::{Path};
use pixiv_api::artworks::{SearchTagPage, PixivArtwork, PixivClient};
use pixiv_api::{PixivError, SearchTag};


#[tokio::main]
async fn main() -> Result<(), PixivError> {
    let config = PixivClient::new(Path::new(env!("CARGO_MANIFEST_DIR")).join("target"));

    let params = SearchTag::portrait("吟霖 100users入り");
    let json: SearchTagPage = params.request(&config, true).await?;
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
        let json = params.with_page(page).request(&config, true).await?;
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

