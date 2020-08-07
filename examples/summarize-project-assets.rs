//! Small example program that prints out a table of projects. For this to work you must set 3 env
//! vars, `SG_SERVER`, `SG_SCRIPT_NAME`, and `SG_SCRIPT_KEY`.
//!
//! Set the `SG_SERVER` environment variable to the url for your shotgun server, eg:
//!
//! ```text
//! export SG_SERVER=https://shotgun.example.com
//! ```
//!
//! `shotgun_rs` also looks at the `CA_BUNDLE` environment variable for when you need a custom CA
//! loaded to access your shotgun server, for example:
//!
//! ```text
//! export CA_BUNDLE=/etc/ssl/my-ca-certs.crt
//! ```
//!
//! Usage:
//!
//! ```text
//! $ cargo run --example summarize-project-assets <project id>
//! ```

use serde_json::{json, Value};
use shotgun_rs::types::{
    Grouping, GroupingDirection, GroupingType, SummaryField, SummaryFieldType,
};
use shotgun_rs::Shotgun;
use std::env;

#[tokio::main]
async fn main() -> shotgun_rs::Result<()> {
    dotenv::dotenv().ok();

    let server = env::var("SG_SERVER").expect("SG_SERVER is required var.");
    let script_name = env::var("SG_SCRIPT_NAME").expect("SG_SCRIPT_NAME is required var.");
    let script_key = env::var("SG_SCRIPT_KEY").expect("SG_SCRIPT_KEY is required var.");

    let project_id: i32 = env::args()
        .nth(1)
        .expect("must specify a project id")
        .parse()
        .expect("invalid project id");

    let sg = Shotgun::new(server, Some(&script_name), Some(&script_key)).expect("SG Client");

    let token = {
        let resp: Value = sg.authenticate_script().await?;
        resp["access_token"].as_str().unwrap().to_string()
    };

    let resp: Value = sg
        .summarize(
            &token,
            "Asset",
            Some(json!([["project", "is", {"type": "Project", "id": project_id}]])),
            Some(vec![SummaryField {
                field: "id".to_string(),
                r#type: SummaryFieldType::Count,
            }]),
            Some(vec![Grouping {
                field: "sg_asset_type".to_string(),
                r#type: GroupingType::Exact,
                direction: Some(GroupingDirection::Asc),
            }]),
            None,
        )
        .await?;

    println!("{}", resp);
    Ok(())
}
