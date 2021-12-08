mod download_file;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let client = reqwest::Client::builder()
        .user_agent("test")
        // .danger_accept_invalid_certs(exec.disable_cert_validation)
        // .danger_accept_invalid_hostnames(exec.disable_hostname_validation)
        // .connection_verbose(exec.verbose)
        .build()
        .unwrap();
    // download_file::download_file(&client, "http://aiweb.cs.washington.edu/research/projects/xmltk/xmldata/data/nasa/nasa.xml", "nasa.xml").await;
    download_file::upload_file(&client, "https://bashupload.com/nasa.xml", "nasa.xml").await;
    // Ok(())
}
