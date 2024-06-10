use tokio::io::AsyncWriteExt;
use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct PixivImage {
    pub urls: PixivImageUrls,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PixivImageUrls {
    #[serde(rename = "thumb_mini")]
    pub mini: String,
    pub small: String,
    pub regular: String,
    pub original: String,
}


impl PixivImage {
    pub async fn download_original(&self, folder: &Path) -> Result<PathBuf, PixivError> {
        let path = if self.width > self.height {
            folder.join("horizontal")
        } else if self.width < self.height {
            folder.join("vertical")
        } else {
            folder.join("square")
        };
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        self.urls.download_original(&path).await
    }
}

impl PixivImageUrls {
    pub async fn download_original(&self, folder: &Path) -> Result<PathBuf, PixivError> {
        let url = self.original.as_str();
        let client = Client::new();
        let mut response = client
            .get(url)
            .header(REFERER, "https://www.pixiv.net/")
            .send()
            .await?;
        let file_name = match url.split('/').last() {
            Some(s) => { folder.join(s) }
            None => panic!("Invalid Image URL"),
        };


        let mut file = File::create(&file_name).await?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes).await?;
        Ok(file_name)
    }
}