use reqwest::Body;
use tokio_util::either::Either;
use std::cmp::min;
use std::fs::File;
use std::io::{Read, Write};
use tokio_util::codec::{BytesCodec, FramedRead};

use futures_util::{StreamExt, TryStreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::io;
use bytes::BytesMut;

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
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    return Ok(());
}

pub async fn upload_small_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // let mut f = File::open(path).unwrap();
    // let mut vec = Vec::new();
    // f.read_to_end(&mut vec);
    // client.post(url).body(vec).send();

    // return Ok(());

    // Reqwest setup
    let res = client
        .post(url)
        .body(Body::from(std::fs::read(path).unwrap()))
        .send()
        .await
        .or(Err(format!("Failed to POST from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Uploading {}", url));

    // upload chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

println!("file size:{}",file.metadata().unwrap().len());
println!("file size:{}",file.metadata().unwrap().len());

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("uploaded {} to {}", url, path));
    return Ok(());
}

pub async fn upload_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    let f = File::open(path).expect("Unable to open file");

    let total_size = f.metadata().unwrap().len();

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(format!("Posting {}", url));


    let file = tokio::fs::File::open(path).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());


    let res=client
    .post(url)
    .body(Body::wrap_stream(stream))
    .send()
    .await;


    pb.finish_with_message(format!("Uploaded {} to {}", url, path));
    return Ok(());
}

async fn upload(client: &Client, url: &str,path: String, sender: Option<mpsc::Sender<usize>) -> Result<(), String> {
    let file = tokio::fs::File::open(path).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());

    let stream = if let Some(mut tx) = sender {
        Either::Left(stream
            .inspect_ok(move |chunk| tx.send(chunk.len()))
        )
    } else {
        Either::Right(stream)
    };

    let body = Body::wrap_stream(stream);

    // not sure where `client` or `url` are defined?
    client.put(url).body(body)
}

struct UploadProgress<R> {
    inner: R,
    bytes_read: usize,
    total: usize,
}

impl<R: Read> Read for UploadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.bytes_read += n;
            println!(
                "Upload progress: {}/{} bytes ({}%)",
                self.bytes_read,
                self.total,
                self.bytes_read * 100 / self.total
            );
            n
        })
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::*;

    // #[tokio::test]
    // async fn test_download() {
    //     let client = reqwest::Client::builder()
    //         .user_agent("test")
    //         .build()
    //         .unwrap();
    //     download_file(
    //         &client,
    //         "http://aiweb.cs.washington.edu/research/projects/xmltk/xmldata/data/nasa/nasa.xml",
    //         "nasa.xml",
    //     )
    //     .await;
    // }
    
    // #[tokio::test]
    // async fn test_upload_small() {
    //     let client = reqwest::Client::builder()
    //         .user_agent("test")
    //         .danger_accept_invalid_certs(true)
    //         .build()
    //         .unwrap();
    //     upload_small_file(&client, "https://bashupload.com/nasa.xml", "nasa.xml").await;
    // }

    #[tokio::test]
    async fn test_upload_small() {
        let client = reqwest::Client::builder()
            .user_agent("test")
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        upload_file(&client, "https://bashupload.com/nasa.xml", "nasa.xml").await;
    }
}
