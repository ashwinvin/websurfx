//! This module handles the execution of search engine parser.
use std::{collections::HashMap, sync::Arc};

use crate::models::{
    aggregation_models::SearchResult,
    client_models::HttpClient,
    engine_models::{EngineError, EngineErrorType, SearchEngine, TimeRelavancy, QueryType},
};
use actix_web::rt::spawn;
use error_stack::Report;
use error_stack::Result;

/// Handler for all upstream engines.
pub struct EngineHandler {
    /// Stores the instances of the search engines to use for fetching results.
    engines: Arc<Vec<Arc<Box<dyn SearchEngine>>>>,
    /// The HTTP client to use for fetching results.
    client: Arc<HttpClient>,
}

/// Represents a vector of hashmaps which contains results and engine returned by each search engine. 
pub type RawResults = Vec<Result<HashMap<String, SearchResult>, EngineError>>;

impl EngineHandler {
    /// Parses names of engines and initialises them for use.
    ///
    /// # Arguments
    ///
    /// * `engine_names - It takes the names of the engines.
    /// * `client` - It takes the initialised HTTP client.
    ///
    /// # Returns
    ///
    /// It returns an option either containing the initialised struct or a none if a given engine name is invalid.
    pub fn new(engine_names: Vec<String>, client: HttpClient) -> Result<Self, EngineError> {
        let mut engines: Vec<Arc<Box<dyn SearchEngine>>> = vec![];

        for engine_name in engine_names {
            let engine: Box<dyn SearchEngine> = match engine_name.to_lowercase().as_str() {
                "duckduckgo" => Box::new(crate::engines::duckduckgo::DuckDuckGo::new()?),
                "searx" => Box::new(crate::engines::searx::Searx::new()?),
                "brave" => Box::new(crate::engines::brave::Brave::new()?),
                _ => {
                    return Err(Report::from(EngineError {
                        error_type: EngineErrorType::NoSuchEngineFound,
                        engine: engine_name.to_string(),
                    }))
                }
            };
            engines.push(Arc::new(engine));
        }

        Ok(Self {
            engines: Arc::new(engines),
            client: Arc::new(client)
        })
    }

    /// Searches given query in each upstream search engine.
    /// 
    /// # Arguments
    ///  * `engine_names` - The engines to use for searching. If none is provided, the search will be done using all
    ///                       active engines.
    ///  * `query` - The string to search.
    ///  * `query_type` - The type of results to search for. 
    ///  * `page` - The page number.
    ///  * `time_relevance` - The required time relevancy of the search.
    /// 
    pub async fn search(
        &self,
        engine_names: Option<Vec<String>>,
        query: &str,
        query_type: QueryType,
        time_relevance: Option<TimeRelavancy>,
        page: u32,
        safe_search: u8,
    ) -> RawResults {
        let mut tasks = Vec::with_capacity(self.engines.len());
        for engine in &*self.engines {
            if let Some(ref engine_names) = engine_names {
                // TODO: Handle invalid engine names provided by the user, currently it silently ignores
                if !engine_names.contains(&engine.get_name().to_owned()) {
                    continue;
                }
            }
            let engine = engine.clone();
            let time_relevance = time_relevance.clone();
            let client = self.client.clone();
            let query = query.to_owned();
            tasks.push(spawn(async move {
                engine
                    .fetch_results(&query, query_type, time_relevance, page, client, safe_search)
                    .await
            }));
        }

        let mut responses = Vec::with_capacity(tasks.len());
        for task in tasks {
            // An error will only be raised when the task panics, here it should technically never panic
            if let Ok(result) = task.await {
                responses.push(result);
            }
        }

        responses
    }
}
