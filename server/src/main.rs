mod analysis;
mod config;
mod data;
mod database;
mod match_info;
mod server_sync;
mod team_info;

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use actix_files::{Files, NamedFile};
use actix_web::http::header::ContentType;
use actix_web::http::{header, StatusCode};
use actix_web::web::Data;
use actix_web::{get, options, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::stream::StreamExt as _;
use serde::Deserialize;
use serde_json::json;
use simplelog::TermLogger;

use crate::data::{Info, MatchInfo};
use crate::database::Database;

#[options("/api/push")]
async fn push_options(_params: ()) -> HttpResponse {
	HttpResponse::build(StatusCode::OK)
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.append_header((header::ACCESS_CONTROL_ALLOW_METHODS, "PUT"))
		.append_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type"))
		.body("")
}

#[put("/api/push")]
async fn push_data(data: Data<Arc<Database>>, mut body: web::Payload) -> HttpResponse {
	// FIXME: for some reason web::Json isn't working.
	let mut bytes = web::BytesMut::new();
	while let Some(item) = body.next().await {
		bytes.extend_from_slice(&item.unwrap());
	}
	let string = String::from_utf8(bytes.to_vec()).unwrap();
	let matches_raw: Vec<serde_json::Value> = serde_json::from_str(&string).unwrap();
	let mut matches = Vec::new();
	for match_raw in matches_raw {
		let match_str = serde_json::to_string(&match_raw).unwrap();
		if let Ok(info_parsed) = serde_json::from_str::<Info>(&match_str) {
			matches.push(info_parsed);
		} else if let Ok(info_parsed) = serde_json::from_str::<MatchInfo>(&match_str) {
			matches.push(Info::MatchInfo(info_parsed));
		}
	}
	if let Err(e) = data.merge_info(&matches) {
		return HttpResponse::build(StatusCode::OK)
			.content_type(ContentType::json())
			.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
			.body(
				serde_json::to_string(&json!({"success": false, "error": e.to_string()})).unwrap(),
			);
	}

	HttpResponse::build(StatusCode::OK)
		.content_type(ContentType::json())
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.body(serde_json::to_string(&json!({"success": true})).unwrap())
}

#[get("/api/pull")]
async fn pull_data(data: Data<Arc<Database>>) -> HttpResponse {
	HttpResponse::build(StatusCode::OK)
		.content_type(ContentType::json())
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.body(
			serde_json::to_string(&json!({"success": true, "data": data.get_info_list()})).unwrap(),
		)
}

#[get("/api/analysis")]
async fn get_analysis(data: Data<Arc<Database>>) -> HttpResponse {
	let teams = analysis::analyze_data(&data);
	HttpResponse::build(StatusCode::OK)
		.content_type(ContentType::json())
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.body(serde_json::to_string(&json!({"success": true, "data": teams})).unwrap())
}

#[derive(Debug, Deserialize)]
struct TeamInfoQueryParams {
	team: u32,
}

#[get("/api/team_info")]
async fn get_team_info(
	data: Data<Arc<Database>>,
	params: web::Query<TeamInfoQueryParams>,
) -> HttpResponse {
	let team = team_info::get_team_info(&data, params.team);
	HttpResponse::build(StatusCode::OK)
		.content_type(ContentType::json())
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.body(serde_json::to_string(&json!({"success": true, "data": team})).unwrap())
}

#[derive(Debug, Deserialize)]
struct MatchInfoQueryParams {
	#[serde(rename = "match")]
	match_number: u32,
}

#[get("/api/match_info")]
async fn get_match_info(
	data: Data<Arc<Database>>,
	params: web::Query<MatchInfoQueryParams>,
) -> HttpResponse {
	if let Some(full_match_info) = match_info::get_match_info(&data, params.match_number) {
		HttpResponse::build(StatusCode::OK)
			.content_type(ContentType::json())
			.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
			.body(
				serde_json::to_string(&json!({"success": true, "data": full_match_info})).unwrap(),
			)
	} else {
		HttpResponse::build(StatusCode::OK)
			.content_type(ContentType::json())
			.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
			.body(
				serde_json::to_string(
					&json!({"success": false, "error": "Not a valid match owo."}),
				)
				.unwrap(),
			)
	}
}

#[get("/api/csv")]
async fn get_csv(data: Data<Arc<Database>>) -> HttpResponse {
	let mut csv = MatchInfo::HEADER.to_string();
	for match_info in data.get_all_matches() {
		csv.push_str(&match_info.unwrap().write_csv_line());
	}
	HttpResponse::build(StatusCode::OK)
		.content_type(ContentType::plaintext())
		.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
		.body(csv)
}

#[derive(Debug, Deserialize)]
struct ImgQueryParams {
	id: u64,
}

#[get("/api/img")]
async fn get_img(params: web::Query<ImgQueryParams>) -> HttpResponse {
	if let Ok(data) = std::fs::read(format!("images/bot-{}.jpeg", params.id)) {
		HttpResponse::build(StatusCode::OK)
			.content_type(ContentType::jpeg())
			.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
			.body(data)
	} else {
		HttpResponse::build(StatusCode::NOT_FOUND)
			.append_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
			.body("Bad image id.")
	}
}

async fn get_index(_req: HttpRequest) -> impl Responder {
	NamedFile::open_async("../client/assets/index.html").await
}

#[tokio::main]
async fn main() {
	let config = config::read_config();
	TermLogger::init(
		simplelog::LevelFilter::Trace,
		simplelog::ConfigBuilder::new()
			.add_filter_allow("automated-scout".to_string())
			.add_filter_allow("actix_web".to_string())
			.build(),
		simplelog::TerminalMode::Stderr,
		simplelog::ColorChoice::Always,
	)
	.unwrap();
	let database = Arc::new(Database::open(&PathBuf::from_str("matches.db").unwrap()));
	if let Some(leader_url) = &config.leader_url {
		let leader_url = leader_url.to_owned();
		let database = database.clone();
		println!("Following leader at {}", leader_url);
		tokio::spawn(async move {
			let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(
				(config.sync_interval / 1000.0) as u64,
			));
			loop {
				interval.tick().await;
				if let Err(e) = server_sync::try_sync(&database, &leader_url) {
					eprintln!("Error syncing with leader: {}", e);
				}
			}
		});
	}
	HttpServer::new(move || {
		let database = database.clone();
		App::new()
			.app_data(Data::new(database))
			.service(push_data)
			.service(push_options)
			.service(pull_data)
			.service(get_csv)
			.service(get_analysis)
			.service(get_img)
			.service(get_team_info)
			.service(get_match_info)
			.service(Files::new("/dist", "../client/dist/").prefer_utf8(true))
			.service(
				Files::new("/", "../client/assets/")
					.prefer_utf8(true)
					.index_file("index.html"),
			)
			.default_service(web::route().to(get_index))
	})
	.bind("0.0.0.0:4421")
	.unwrap()
	.run()
	.await
	.unwrap();
}
