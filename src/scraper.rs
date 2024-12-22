use std::{path::PathBuf, str::FromStr, sync::LazyLock};

use kuchikiki::traits::TendrilSink;

use crate::configuration::{Paper, PaperType, Season, SyllabusCode};

const BASE_URL: &str = "https://papers.gceguide.cc/a-levels/";
static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

#[derive(Debug)]
pub enum RequestError {
    ReqwestError(reqwest::Error),
    NotFound(&'static str),
    TokioError(std::io::Error),
}

#[derive(Debug, Clone)]
pub struct PaperRequest {
    pub syllabus: SyllabusCode,
    pub year: String,
    pub seasons: Vec<Season>,
    pub papers: Vec<PaperType>,
}


pub async fn get_all_papers(request: &PaperRequest) -> Vec<Paper> {
    let url = format!(
        "{}{}/{}",
        BASE_URL, request.syllabus.access_slug, request.year
    );
    info!("Requesting papers from: {}", url);

    let client = REQWEST_CLIENT.get(url).send().await;
    let res = match client {
        Ok(response) => {
            let body = response.text().await;
            match body {
                Ok(body) => Ok(body),
                Err(e) => Err(RequestError::ReqwestError(e)),
            }
        }
        Err(e) => Err(RequestError::ReqwestError(e)),
    };
    if let Err(e) = res {
        error!("Error: {:?}", e);
        return vec![];
    }

    let body = res.unwrap();
    let document = kuchikiki::parse_html().one(body);

    let paper_nodes = document.document_node.select(".name");
    let paper_content_list: Vec<String> = match paper_nodes {
        Ok(paper_nodes) => paper_nodes
            .map(|node| node.as_node().text_contents())
            .collect(),
        Err(_) => return vec![],
    };

    let mut papers = vec![];
    for paper_content in paper_content_list {
        let paper = Paper::from_str(&paper_content);
        match paper {
            Ok(paper) => {
                if request.papers.contains(&paper.paper_type) {
                    papers.push(paper);
                }
            },
            Err(e) => error!("Error parsing paper for {}: {:?}", &paper_content, e),
        };
    }

    
    papers
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

pub async fn save_paper(syllabus: &SyllabusCode, paper: &Paper, output_file: &PathBuf) {
    let url = format!(
        "{}{}/{}/{}",
        BASE_URL, syllabus.access_slug, paper.year, paper.get_ref_filename(syllabus)
    );
    info!("Requesting paper from: {}", url);

    let client = REQWEST_CLIENT.get(url).send().await;
    let res = match client {
        Ok(response) => {
            let body = response.bytes().await;
            match body {
                Ok(body) => Ok(body),
                Err(e) => Err(RequestError::ReqwestError(e)),
            }
        }
        Err(e) => Err(RequestError::ReqwestError(e)),
    };

    if let Err(e) = res {
        error!("Error: {:?}", e);
        return;
    }

    let body = res.unwrap();
    match std::fs::write(output_file, body.as_ref()) {
        Ok(_) => info!("Saved paper to: {:?}", output_file),
        Err(e) => error!("Error saving paper: {:?}", e),
    }
}