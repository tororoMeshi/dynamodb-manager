use anyhow::Result;
use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct TableInfo {
    table_name: String,
    attributes: Vec<AttributeInfo>,
}

#[derive(Serialize, Deserialize)]
struct AttributeInfo {
    attribute_name: String,
    type_hint: String,
    description: Option<String>,
    last_seen_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct UpdateMemoRequest {
    description: String,
}

struct DynamoDBService {
    client: Client,
}

impl DynamoDBService {
    async fn new() -> Result<Self> {
        let config = load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Ok(Self { client })
    }

    async fn list_tables(&self) -> Result<Vec<String>> {
        let resp = self.client.list_tables().send().await?;
        Ok(resp.table_names().to_vec())
    }

    async fn get_table_attributes(&self, table_name: &str) -> Result<Vec<AttributeInfo>> {
        let resp = self
            .client
            .describe_table()
            .table_name(table_name)
            .send()
            .await?;

        let mut attributes = Vec::new();

        if let Some(table) = resp.table() {
            for attr_def in table.attribute_definitions() {
                let attr_name = attr_def.attribute_name();
                let attr_type = match attr_def.attribute_type() {
                    aws_sdk_dynamodb::types::ScalarAttributeType::S => "string",
                    aws_sdk_dynamodb::types::ScalarAttributeType::N => "number",
                    aws_sdk_dynamodb::types::ScalarAttributeType::B => "binary",
                    _ => "unknown",
                };

                let description = self.get_attribute_memo(table_name, attr_name).await?;

                attributes.push(AttributeInfo {
                    attribute_name: attr_name.to_string(),
                    type_hint: attr_type.to_string(),
                    description,
                    last_seen_at: Some(chrono::Utc::now().to_rfc3339()),
                });
            }
        }

        Ok(attributes)
    }

    async fn get_attribute_memo(
        &self,
        table_name: &str,
        attribute_name: &str,
    ) -> Result<Option<String>> {
        let key = HashMap::from([
            (
                "table_name".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(table_name.to_string()),
            ),
            (
                "attribute_name".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(attribute_name.to_string()),
            ),
        ]);

        let resp = self
            .client
            .get_item()
            .table_name("dynamodb-admin-meta")
            .set_key(Some(key))
            .send()
            .await?;

        if let Some(item) = resp.item() {
            if let Some(description) = item.get("description") {
                if let Ok(desc_str) = description.as_s() {
                    return Ok(Some(desc_str.clone()));
                }
            }
        }

        Ok(None)
    }

    async fn update_attribute_memo(
        &self,
        table_name: &str,
        attribute_name: &str,
        description: &str,
    ) -> Result<()> {
        let item = HashMap::from([
            (
                "table_name".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(table_name.to_string()),
            ),
            (
                "attribute_name".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(attribute_name.to_string()),
            ),
            (
                "description".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(description.to_string()),
            ),
            (
                "last_seen_at".to_string(),
                aws_sdk_dynamodb::types::AttributeValue::S(chrono::Utc::now().to_rfc3339()),
            ),
        ]);

        self.client
            .put_item()
            .table_name("dynamodb-admin-meta")
            .set_item(Some(item))
            .send()
            .await?;

        Ok(())
    }

    async fn get_sample_data(
        &self,
        table_name: &str,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>> {
        let resp = self
            .client
            .scan()
            .table_name(table_name)
            .limit(5)
            .send()
            .await?;

        let mut samples = Vec::new();
        for item in resp.items() {
            let mut sample = HashMap::new();
            for (key, value) in item {
                let json_value = match value {
                    aws_sdk_dynamodb::types::AttributeValue::S(s) => json!(s),
                    aws_sdk_dynamodb::types::AttributeValue::N(n) => {
                        if let Ok(num) = n.parse::<i64>() {
                            json!(num)
                        } else if let Ok(float) = n.parse::<f64>() {
                            json!(float)
                        } else {
                            json!(n)
                        }
                    }
                    aws_sdk_dynamodb::types::AttributeValue::Bool(b) => json!(b),
                    aws_sdk_dynamodb::types::AttributeValue::Null(_) => json!(null),
                    _ => json!(format!("{:?}", value)),
                };
                sample.insert(key.clone(), json_value);
            }
            samples.push(sample);
        }

        Ok(samples)
    }
}

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let path = request.uri().path();
    let method = request.method().as_str();

    let db_service = DynamoDBService::new()
        .await
        .map_err(|e| Error::from(format!("Failed to create DynamoDB service: {e}")))?;

    let response = match (method, path) {
        ("GET", "/tables") => {
            let tables = db_service
                .list_tables()
                .await
                .map_err(|e| Error::from(format!("Failed to list tables: {e}")))?;

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json!(tables).to_string().into())?
        }
        ("GET", path) if path.starts_with("/tables/") && path.ends_with("/sample") => {
            let table_name = path
                .trim_start_matches("/tables/")
                .trim_end_matches("/sample");

            let samples = db_service
                .get_sample_data(table_name)
                .await
                .map_err(|e| Error::from(format!("Failed to get sample data: {e}")))?;

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json!(samples).to_string().into())?
        }
        ("GET", path) if path.starts_with("/tables/") => {
            let table_name = path.trim_start_matches("/tables/");

            let attributes = db_service
                .get_table_attributes(table_name)
                .await
                .map_err(|e| Error::from(format!("Failed to get table attributes: {e}")))?;

            let table_info = TableInfo {
                table_name: table_name.to_string(),
                attributes,
            };

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json!(table_info).to_string().into())?
        }
        ("PUT", path) if path.starts_with("/tables/") => {
            let parts: Vec<&str> = path.trim_start_matches("/tables/").split('/').collect();
            if parts.len() != 2 {
                return Ok(Response::builder()
                    .status(400)
                    .header("Access-Control-Allow-Origin", "*")
                    .body("Invalid path format".into())?);
            }

            let table_name = parts[0];
            let attribute_name = parts[1];

            let body = request.body();
            let body_str = std::str::from_utf8(body)
                .map_err(|e| Error::from(format!("Invalid UTF-8 in request body: {e}")))?;

            let update_request: UpdateMemoRequest = serde_json::from_str(body_str)
                .map_err(|e| Error::from(format!("Invalid JSON in request body: {e}")))?;

            db_service
                .update_attribute_memo(table_name, attribute_name, &update_request.description)
                .await
                .map_err(|e| Error::from(format!("Failed to update memo: {e}")))?;

            Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(json!({"success": true}).to_string().into())?
        }
        ("OPTIONS", _) => Response::builder()
            .status(204)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body("".into())?,
        _ => Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body("Not Found".into())?,
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
