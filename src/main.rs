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
    target_branch: Vec<String>,
}

fn get_merge_request(
    private_token: &String,
    gitlab_url: &String,
    project_id: u32,
    source_mr_id: u32,
) -> ureq::serde_json::Value {
    let path = format!(
        "{}/api/v4/projects/{}/merge_requests/{}",
        gitlab_url, project_id, source_mr_id
    );

    let response = ureq::get(&path)
        .set("PRIVATE-TOKEN", &private_token)
        .call()
        .unwrap();
    let mr: ureq::serde_json::Value = response.into_json().unwrap();
    // println!("{}", mr);
    mr
}

fn create_merge_request(
    private_token: &String,
    gitlab_url: &String,
    project_id: u32,
    target_branch: &String,
    mr: &ureq::serde_json::Value,
) -> ureq::serde_json::Value {
    let path = format!(
        "{}/api/v4/projects/{}/merge_requests",
        gitlab_url, project_id
    );

    let response = ureq::post(&path)
        .set("PRIVATE-TOKEN", &private_token)
        .send_json(ureq::json!({
            "source_branch": mr["source_branch"],
            "target_branch": target_branch,
            "title": mr["title"],
            "description": mr["description"],
        }))
        .unwrap();
    let response: ureq::serde_json::Value = response.into_json().unwrap();
    // println!("{}", response);
    response
}

fn copy_merge_request(private_token: &String, config: &Config) {
    let mr = get_merge_request(
        private_token,
        &config.gitlab_url,
        config.project_id,
        config.source_mr_id,
    );

    for target in &config.target_branch {
        let response = create_merge_request(
            private_token,
            &config.gitlab_url,
            config.project_id,
            target,
            &mr,
        );
        println!("create merge_request {}", response["web_url"]);
    }
}

fn main() {
    let args = Args::parse();

    let config = {
        let file = File::open(args.config).unwrap();
        let reader = BufReader::new(file);
        let config: Config = ureq::serde_json::from_reader(reader).unwrap();
        config
    };

    let private_token = dotenv::var("ACCESS_TOKEN").unwrap();

    copy_merge_request(&private_token, &config);
}
