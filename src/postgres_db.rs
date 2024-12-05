use tokio_postgres::{ Client, NoTls, Connection, Error, Row };
use tokio_postgres::types::ToSql;
use postgres_native_tls::MakeTlsConnector;

use std::{ thread, time::Duration };
use native_tls::TlsConnector;

pub struct PostgresDb {
    client: Client,
}

impl PostgresDb {
    pub async fn new(connection_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        const MAX_RETRIES: u32 = 2;
        const RETRY_DELAY: Duration = Duration::from_secs(3);

        let mut retry_count = 0;

        let tls_connector = TlsConnector::builder().build()?;
        let connector = MakeTlsConnector::new(tls_connector);

        loop {
            match tokio_postgres::connect(connection_url, connector.clone()).await {
                Ok((client, connection)) => {
                    log::info!("PostgreSQL connection established successfully");
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                    return Ok(PostgresDb { client });
                }
                Err(e) => {
                    retry_count += 1;
                    log::warn!("PostgreSQL connection attempt {} failed: {}", retry_count, e);

                    if retry_count >= MAX_RETRIES {
                        log::error!("Max retries reached. Using fallback mechanism");
                        return Ok(PostgresDb {
                            client: tokio_postgres::connect(
                                "postgresql://admin:password123@localhost:5432/rust_api",
                                NoTls
                            ).await?.0,
                        });
                    }

                    tokio::time::sleep(RETRY_DELAY).await;
                }
            }
        }
    }

    pub async fn execute(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)]
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client.execute(query, params).await?;
        Ok(())
    }

    pub async fn query<T>(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
        mapper: fn(&Row) -> Result<T, Box<dyn std::error::Error>>
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
        where T: std::fmt::Debug
    {
        let rows = self.client.query(query, params).await?;
        let mut results = Vec::new();
        for row in rows {
            results.push(mapper(&row)?);
        }

        Ok(results)
    }
}
