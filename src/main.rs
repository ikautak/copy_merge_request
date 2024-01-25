use clap::Parser;
use dotenv;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use ureq;

#[derive(Parser, Debug)]
#[clap(version, about = "copy gitlab merge-request")]
struct Args {
    #[clap(short, long)]
    config: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    gitlab_url: String,
    project_id: u32,
    source_mr_id: u32,
    target_branches: Vec<String>,
}

fn get_merge_request(
    access_token: &str,
    gitlab_url: &str,
    project_id: u32,
    source_mr_id: u32,
) -> Result<ureq::serde_json::Value, ureq::Error> {
    let path = format!(
        "{}/api/v4/projects/{}/merge_requests/{}",
        gitlab_url, project_id, source_mr_id
    );

    let response = ureq::get(&path)
        .set("PRIVATE-TOKEN", &access_token)
        .call()?;

    let mr: ureq::serde_json::Value = response.into_json()?;
    Ok(mr)
}

fn post_merge_request(
    access_token: &str,
    gitlab_url: &str,
    project_id: u32,
    target_branch: &str,
    mr: &ureq::serde_json::Value,
) -> Result<ureq::serde_json::Value, ureq::Error> {
    let path = format!(
        "{}/api/v4/projects/{}/merge_requests",
        gitlab_url, project_id
    );

    let response = ureq::post(&path)
        .set("PRIVATE-TOKEN", &access_token)
        .send_json(ureq::json!({
            "source_branch": mr["source_branch"],
            "target_branch": target_branch,
            "title": mr["title"],
            "description": mr["description"],
        }))?;

    let response: ureq::serde_json::Value = response.into_json()?;
    Ok(response)
}

fn copy_merge_request(access_token: &str, config: &Config) {
    let mr = match get_merge_request(
        access_token,
        &config.gitlab_url,
        config.project_id,
        config.source_mr_id,
    ) {
        Ok(mr) => mr,
        Err(e) => {
            eprintln!("Failed to get merge request: {}", e);
            return;
        }
    };

    for target_branch in &config.target_branches {
        match post_merge_request(
            access_token,
            &config.gitlab_url,
            config.project_id,
            target_branch,
            &mr,
        ) {
            Ok(response) => println!("{} {}", target_branch, response["web_url"]),
            Err(e) => {
                eprintln!("Failed to post merge request: {}", e);
                return;
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let config = {
        let file = File::open(args.config).expect("Failed to open config");
        let reader = BufReader::new(file);
        let config: Config = ureq::serde_json::from_reader(reader).expect("Failed to parse config");
        config
    };

    let access_token = dotenv::var("ACCESS_TOKEN").unwrap();

    copy_merge_request(&access_token, &config);
}
