use std::sync::LazyLock;

use kuchikiki::traits::TendrilSink;

use crate::configuration::SyllabusCode;

const BASE_URL: &str = "https://papers.gceguide.cc/a-levels/";
static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

#[derive(Debug)]
pub enum RequestError {
    ReqwestError(reqwest::Error),
    NotFound(&'static str),
    TokioError(std::io::Error),
}

pub async fn get_all_years(syllabus: &SyllabusCode) -> Result<Vec<String>, RequestError> {
    let url = format!("{}{}", BASE_URL, syllabus.access_slug);

    info!("Requesting years from: {}", url);
    // need to run blocking code here
    let client = REQWEST_CLIENT.get(url).send().await;
    let res: Result<String, RequestError> = match client {
        Ok(response) => {
            let body: Result<String, reqwest::Error> = response.text().await;
            match body {
                Ok(body) => Ok(body),
                Err(e) => Err(RequestError::ReqwestError(e)),
            }
        }
        Err(e) => Err(RequestError::ReqwestError(e)),
    };

    match res {
        Ok(body) => {
            let document = kuchikiki::parse_html().one(body);
            let year_nodes = document.document_node.select(".name");
            match year_nodes {
                Ok(year_nodes) => {
                    let year_content_list: Vec<String> = year_nodes
                        .map(|node| node.as_node().text_contents())
                        .filter(|year| year.len() == 4) // Easy way to verify it's a year.
                        .collect();
                    if year_content_list.is_empty() {
                        return Err(RequestError::NotFound("No years data found."));
                    }
                    Ok(year_content_list)
                }
                Err(_) => Err(RequestError::NotFound("No years elements found.")),
            }
        }
        Err(e) => Err(e),
    }
}
