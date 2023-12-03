//! This module provides the error enum to handle different errors associated while requesting data from
//! the upstream search engines with the search query provided by the user.

use super::aggregation_models::SearchResult;
use super::client_models::HttpClient;
use enumflags2::bitflags;
use error_stack::Result;
use std::{collections::HashMap, fmt, sync::Arc};

/// A custom error type used for handling engine associated errors.
#[derive(Debug)]
pub struct EngineError {
    /// The type of error that has occured.
    pub error_type: EngineErrorType,
    /// The name of the search engine that raised this errror.
    pub engine: String,
}

/// A custom error type used for indicating the type of exception.
#[derive(Debug)]
pub enum EngineErrorType {
    /// No matching engine found
    NoSuchEngineFound,
    /// This variant handles all request related errors like forbidden, not found,
    /// etc.
    EmptyResultSet,
    /// This variant handles the not results found error provide by the upstream
    /// search engines.
    RequestError,
    ///  This variant handles all the errors which are unexpected or occur rarely
    /// and are errors mostly related to failure in initialization of HeaderMap,
    /// Selector errors and all other errors occurring within the code handling
    /// the `upstream search engines`.
    UnexpectedError,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error_type {
            EngineErrorType::NoSuchEngineFound => {
                write!(f, "No such engine with the name '{0}' found", self.engine)
            }
            EngineErrorType::EmptyResultSet => {
                write!(f, "The upstream search engine returned an empty result set")
            }
            EngineErrorType::RequestError => {
                write!(
                    f,
                    "Error occurred while requesting data from upstream search engine"
                )
            }
            EngineErrorType::UnexpectedError => {
                write!(f, "An unexpected error occurred while processing the data")
            }
        }
    }
}

impl std::error::Error for EngineError {}

// TODO: Should names be standardised? such as should everything related to search be prefixed search?

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// Represents type of results the query requested.
pub enum QueryType {
    /// Text search results
    Text = 0b00001,
    /// Video search results
    Video = 0b00010,
    /// Image search results
    Image = 0b00100,
    /// File search results. This also covers torrent files.
    File = 0b01000,
    /// Autocompletion results.
    AutoCompletion = 0b10000,
}

/// The time relevancy that the results must be within.
#[derive(Debug, Clone)]
pub enum TimeRelavancy {
    /// 
    Anytime,
    /// 
    LastDay,
    /// 
    LastWeek,
    /// 
    LastMonth,
    /// 
    LastYear,
}

/// A trait to define common behavior for all search engines.
#[async_trait::async_trait]
pub trait SearchEngine: Sync + Send {
    /// This function should return the name of the search engine
    fn get_name(&self) -> &'static str;

    /// This function should return the types of search results it can provide.
    fn get_query_types(&self) -> QueryType;

    /// This function scrapes results from the upstream engine and puts all the scraped results like
    /// title, visiting_url (href in html),engine (from which engine it was fetched from) and description
    /// in a RawSearchResult and then adds that to HashMap whose keys are url and values are RawSearchResult
    /// struct and then returns it within a Result enum.
    ///
    /// # Arguments
    ///
    /// * `query` - Takes the user provided query to query to the upstream search engine with.
    //  * `query_type` - The type of results to search for. 
    ///  * `page` - The page number.
    ///  * `time_relevance` - The required time relevancy of the search.
    ///
    /// # Errors
    ///
    /// Returns an `EngineError` if the user is not connected to the internet or if their is failure to
    /// reach the above `upstream search engine` page or if the `upstream search engine` is unable to
    /// provide results for the requested search query and also returns error if the scraping selector
    /// or HeaderMap fails to initialize.
    async fn fetch_results(
        &self,
        query: &str,
        query_type: QueryType,
        time_relevance: Option<TimeRelavancy>,
        page: u32,
        client: Arc<HttpClient>,
        safe_search: u8,
    ) -> Result<HashMap<String, SearchResult>, EngineError>;
}
