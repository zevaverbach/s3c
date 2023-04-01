use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3, PutObjectRequest};
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: upload_to_s3 <filename> <bucket_name> <metadata> [key]");
        return Ok(());
    }
    let filename = &args[1];
    let bucket_name = &args[2];
    let metadata = &args[3];
    let key = if args.len() >= 5 {
        &args[4]
    } else {
        // If 'key' is not provided, use the file name without path
        Path::new(filename).file_name().unwrap().to_str().unwrap()
    };

    // Read file and metadata
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let metadata_vec: Vec<&str> = metadata.split(",").collect::<Vec<&str>>();
    let mut object_metadata = HashMap::new();
    for i in (0..metadata_vec.len()).step_by(2) {
        object_metadata.insert(metadata_vec[i].to_string(), metadata_vec[i + 1].to_string());
    }

    // Upload file to S3
    let client = S3Client::new(Region::default());
    let request = PutObjectRequest {
        bucket: bucket_name.to_string(),
        key: key.to_string(),
        body: Some(contents.into()),
        metadata: Some(object_metadata),
        ..Default::default()
    };
    match client.put_object(request).await {
        Ok(_) => println!("File uploaded successfully"),
        Err(e) => println!("Error uploading file: {}", e),
    }

    Ok(())
}

