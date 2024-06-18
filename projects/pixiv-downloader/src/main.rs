use std::path::{Path};
use pixiv_api::artworks::{SearchTagPage, PixivArtwork, PixivClient};
use pixiv_api::{PixivError, SearchIllustration};


#[tokio::main]
async fn main() -> Result<(), PixivError> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut config = PixivClient::new(here.join("target"));
    config.use_cookie_from_path(here.join("src/COOKIE.TXT"))?;

    let params = SearchIllustration::portrait("吟霖 100users入り");
    let json: SearchTagPage = params.request(&config, true).await?;
    for data in json.illusion.data.clone() {
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
    for page in 2..=json.illusion.last_page {
        let json = params.with_page(page).request(&config, true).await?;
        for data in json.illusion.data {
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

