use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

pub struct AzureStorage {
    account_name: String,
    blob_client: BlobServiceClient,
}

impl AzureStorage {
    pub fn new(account: String, access_key: String) -> Self {
        let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
        let blob_client = BlobServiceClient::new(account.clone(), storage_credentials);

        Self {
            account_name: account,
            blob_client,
        }
    }

    pub async fn upload_blob(
        &self,
        container: &str,
        blob_name: &str,
        data: Vec<u8>
    ) -> Result<String, Box<dyn std::error::Error>> {
        let container_client = self.blob_client.container_client(container);
        let blob_client = container_client.blob_client(blob_name);

        blob_client.put_block_blob(data).content_type("application/octet-stream").await?;

        let blob_url = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            self.account_name,
            container,
            blob_name
        );

        Ok(blob_url)
    }

    pub async fn download_blob(
        &self,
        container: &str,
        blob_name: &str
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let container_client = self.blob_client.container_client(container);
        let blob_client = container_client.blob_client(blob_name);

        let data = blob_client.get_content().await?;

        Ok(data)
    }
}
