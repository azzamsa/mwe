use aws_sdk_s3::Client;
use aws_types::region::Region;
use http::Uri;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let secret_key = "secret";
    let access_key = "access";
    let url = "https://storage.company.net";

    let cred = aws_sdk_s3::Credentials::new(access_key, secret_key, None, None, "custom");
    let url: Uri = url.parse().unwrap();
    let endpoint = aws_sdk_s3::Endpoint::immutable(url);
    // Region is not used, but required by the SDK's API
    let region = Region::new("us-west-2");
    let config = aws_sdk_s3::config::Builder::new()
        .endpoint_resolver(endpoint)
        .region(region)
        .credentials_provider(cred)
        .build();
    let client = Client::from_conf(config);

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{}", bucket.name().unwrap_or_default());
    }

    println!();
    println!("Found {} buckets.", num_buckets);

    Ok(())
}
