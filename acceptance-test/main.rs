mod steps;

use std::{env, fs};

use cucumber::{writer::{self}, World as _, WriterExt};

#[derive(Debug, cucumber::World)]
pub struct World {
    pub host: String,
    pub endpoint: String,
    pub response: Option<reqwest::Response>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            host: std::env::var("API_HOST").unwrap_or_else(|_| "http://localhost:8000/api".to_string()),
            endpoint: String::new(),
            response: None,
        }
    }
}

#[tokio::main]
async fn main() {
    let out_dir = env::current_dir().unwrap();
    let out_dir = out_dir.to_str().unwrap().split("acceptance-test").next().unwrap();
        
    let xml_file = fs::File::create(format!("{}/cucumber-report.xml", out_dir)).unwrap();
    let json_file = fs::File::create(format!("{}/cucumber-report.json", out_dir)).unwrap();

    World::cucumber()
    .with_writer(
        writer::Basic::stdout() // And output to STDOUT.
            .summarized()       // Simultaneously, add execution summary.
            .tee::<World, _>(writer::JUnit::for_tee(xml_file, 0))
            .tee::<World, _>(writer::Json::for_tee(json_file)) // Then, output to XML file.
            .normalized()       // First, normalize events order.
    )
        .run("acceptance-test/features")
        .await;
}