Learning Rust - Backend RESTful API Development
==========
# rust_api: A Rust-based RESTful API

This project implements a RESTful API using Rust.  It leverages [Ollama](https://ollama.org/) for large language model (LLM) interaction and PostgreSQL for persistent data storage.

## Functionality

The API provides endpoints for:

* **Generating text:**  Uses the integrated Ollama client to send prompts to an LLM and return generated text.  Various parameters (temperature, max tokens) control the generation process.
* **Database Interaction:**  Connects to a PostgreSQL database for data persistence (CRUD operations).

## Architecture

The application is structured as follows:

* **Ollama Integration (`ollama.rs`):** Handles communication with the Ollama API for LLM interaction.  This module provides an abstraction layer for interacting with the LLM, allowing for easy switching between models if needed.
  
```
    use ollama_rs::{
        generation::{completion::request::GenerationRequest, options::GenerationOptions},
        Ollama,
    };


    pub struct OllamaAI {
        client: Ollama,
    }

    impl OllamaAI {
        pub fn new() -> Self {
            Self {
                client: Ollama::default(),
            }
        }

        pub async fn generate_text(
            &self,
            model: &str,
            prompt: &str,
            temperature: f32,
            max_tokens: u32,
        ) -> Result<String, Box<dyn std::error::Error>> {
            let options = GenerationOptions::default().temperature(temperature).top_k(max_tokens);
            let request = GenerationRequest::new(model.to_string(), prompt.to_string()).options(options);
        
            match self.client.generate(request).await {
                Ok(response) => Ok(response.response),
                Err(e) => Err(Box::new(e))
            }
        }
        
    }
```
* **PostgreSQL Integration:** Manages database connections and performs database operations.  This includes handling database connections, executing queries, and processing results.  Consideration is given to connection pooling for efficiency.

```
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
```
* **API Endpoints:** (Implementation details to be added). Defines and implements the RESTful endpoints using a suitable framework (e.g., Actix Web). Each endpoint handles requests, interacts with the Ollama and PostgreSQL modules, and returns appropriate responses.

* **Error Handling:** Comprehensive error handling is implemented throughout the application to gracefully manage issues such as database connection failures, invalid user input, and LLM errors.


## Technologies Used

* **Rust:** Programming language.
* **Ollama:**  LLM interaction library.
* **Tokio:** Asynchronous runtime for concurrent operations.
* **Tokio-Postgres:** Asynchronous PostgreSQL driver.
* **Actix Web (or similar):**  Web framework for building the RESTful API (choice to be specified).
* **Serde:**  Serialization/deserialization library for JSON handling.




### Future Enhancements
* Implement robust authentication and authorization (JWT).
* Explore other LLMs beyond Llama 2.
* Add rate limiting to prevent abuse.