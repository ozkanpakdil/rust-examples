use reqwest::Body;
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use tokio_util::codec::{BytesCodec, FramedRead};

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

pub async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    return Ok(());
}

pub async fn upload_small_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    client
        .post(url)
        .body(Body::from(std::fs::read(path).unwrap()))
        .send()
        .await
        .or(Err(format!("Failed to POST from '{}'", &url)))?;
    return Ok(());
}

pub async fn upload_file(
    client: &Client,
    url: &'static str,
    path: &'static str,
) -> Result<(), String> {
    let f = File::open(path).expect("Unable to open file");

    let total_size = f.metadata().unwrap().len();

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Posting {}", url));

    let mut uploaded = 0;

    let file = tokio::fs::File::open(path).await.unwrap();
    let mut reader_stream = FramedRead::new(file, BytesCodec::new());
    let async_stream = async_stream::stream! {
        while let Some(chunk) = reader_stream.next().await {
            if let Ok(chunk) = &chunk {
                let new = min(uploaded + (chunk.len() as u64), total_size);
                uploaded = new;
                pb.set_position(new);
            }
            yield chunk;
        }
        pb.finish_with_message(format!("Uploaded {} to {}", url, path));
    };

    client
        .post(url)
        .body(Body::wrap_stream(async_stream))
        .send()
        .await
        .ok();

    return Ok(());
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use std::{fs, path::Path};

    use super::*;

    async fn setup() {
        if !Path::new("nasa.xml").is_file() {
            println!("downloading the nasa xml.");
            let client = reqwest::Client::builder()
                .user_agent("test")
                .build()
                .unwrap();
            download_file(
                &client,
                "http://aiweb.cs.washington.edu/research/projects/xmltk/xmldata/data/nasa/nasa.xml",
                "nasa.xml",
            )
            .await;
        }
    }

    #[tokio::test]
    async fn test_upload_small() {
        fs::write("test.txt", "some data for uploading.").expect("Unable to write file");
        let client = reqwest::Client::builder()
            .user_agent("test")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        upload_small_file(&client, "https://bashupload.com/test.txt", "test.txt").await;
        fs::remove_file("test.txt");
    }

    #[tokio::test]
    async fn test_upload() {
        if !fs::metadata("nasa.xml").is_ok() {
            setup().await;
        }
        let client = reqwest::Client::builder()
            .user_agent("test")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        upload_file(&client, "https://bashupload.com/nasa.xml", "nasa.xml").await;
        fs::remove_file("nasa.xml");
    }
}
