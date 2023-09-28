/// https://github.com/awslabs/aws-sdk-rust/blob/main/sdk/s3/tests/reconnects.rs

use aws_sdk_s3::Client;
use aws_types::region::Region;
use aws_sdk_s3::config::Credentials;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let secret_key = "secret";
    let access_key = "access";

    let cred = Credentials::new(access_key, secret_key, None, None, "custom");
    // Region is not used, but required by the SDK's API
    let config = aws_sdk_s3::config::Builder::new()
        .region(Region::new("us-west-2"))
        .credentials_provider(cred)
        .endpoint_url("https://storage.company.com")
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
