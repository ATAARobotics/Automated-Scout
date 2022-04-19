use crate::{Database, Info};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(from = "WebResultInner<T>")]
struct WebResult<T>(pub Result<Option<T>, String>);

#[derive(Debug, Deserialize)]
struct WebResultInner<T> {
	success: bool,
	data: Option<T>,
	error: Option<String>,
}

impl<T> From<WebResultInner<T>> for WebResult<T> {
	fn from(inner: WebResultInner<T>) -> Self {
		WebResult(if inner.success {
			Ok(inner.data)
		} else {
			Err(inner.error.unwrap_or_else(|| "No error".to_string()))
		})
	}
}

pub fn try_sync(database: &Database, url: &str) -> Result<(), String> {
	serde_json::from_str::<WebResult<()>>(
		&ureq::put(&format!("{}/api/push", url))
			.set("Content-Type", "application/json")
			.send_json(&database.get_info_list())
			.map_err(|e| e.to_string())?
			.into_string()
			.map_err(|e| e.to_string())?,
	)
	.map_err(|e| e.to_string())?
	.0?;
	let new_info = serde_json::from_str::<WebResult<Vec<Info>>>(
		&ureq::get(&format!("{}/api/pull", url))
			.call()
			.map_err(|e| e.to_string())?
			.into_string()
			.map_err(|e| e.to_string())?,
	)
	.map_err(|e| e.to_string())?
	.0?
	.ok_or_else(|| "No data".to_string())?;
	database.merge_info(&new_info).map_err(|e| e.to_string())?;
	ureq::get(&format!("{}/api/pull", url)).call().unwrap();
	println!("Synchronized with {}.", url);
	Ok(())
}
