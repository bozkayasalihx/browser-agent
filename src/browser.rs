use anyhow::{anyhow, Result};
use chromiumoxide::{
    fetcher::BrowserFetcherRevisionInfo, Browser, BrowserConfig, BrowserFetcher,
    BrowserFetcherOptions,
};
use std::path::Path;
use tokio_stream::StreamExt;

pub async fn init(browser_path: &Path, user_data_dir: &Path, headless: bool) -> Result<Browser> {
    let browser_info = ensure_browser(browser_path).await?;

    let mut config = BrowserConfig::builder()
        .user_data_dir(user_data_dir)
        .chrome_executable(browser_info.executable_path);

    if headless {
        config = config.with_head();
    }

    let (browser, mut handler) = Browser::launch(config.build().map_err(|e| anyhow!(e))?).await?;

    tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    Ok(browser)
}

async fn ensure_browser(path: &Path) -> Result<BrowserFetcherRevisionInfo> {
    let fetcher = BrowserFetcher::new(BrowserFetcherOptions::builder().with_path(path).build()?);

    Ok(fetcher.fetch().await?)
}