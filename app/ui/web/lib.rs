/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod config;
mod routes;

use std::path::PathBuf;

use actix_web::{
	error,
	web::{self, Data},
	App, Error, HttpRequest, HttpResponse, HttpServer,
};
use awc::Client;
use lazy_static::lazy_static;
use tera::Tera;
use url::Url;

use self::config::ClientWebConfig;

// ------ //
// Static //
// ------ //

lazy_static! {
	pub static ref VIEWS: Tera = {
		let mut tera = match Tera::new("app/ui/web/views/**/*.html") {
			| Ok(template) => template,
			| Err(err) => panic!("Analyse des vues échouée: {}", err),
		};
		tera.autoescape_on(vec![".html"]);
		tera
	};
}

// --------- //
// Structure //
// --------- //

pub struct WEB;

// -------------- //
// Implémentation //
// -------------- //

impl WEB {
	// Crée un serveur WEB.
	pub async fn launch(
		config_filename: impl Into<PathBuf>,
	) -> std::io::Result<()> {
		let client_web_cfg =
			Data::new(fs::TOMLFileLoader::<ClientWebConfig>::load(
				config_filename.into(),
			)?);

		let cfg_w1 = client_web_cfg.clone();

		let addr = format!(
			"{}:{}",
			client_web_cfg.server.host, client_web_cfg.server.port
		);
		println!();
		println!("\thttp://{}", &addr);
		println!();

		HttpServer::new(move || {
			let mut app = App::new();

			if let Some(forward_url) = &client_web_cfg.server.proxy {
				let forward_url = Url::parse(forward_url).unwrap();
				app = app
					.app_data(web::Data::new(Client::default()))
					.app_data(web::Data::new(forward_url))
					.default_service(web::to(Self::forward));
			}

			app.app_data(cfg_w1.clone())
				.app_data(web::Data::new(VIEWS.clone()))
				.service(
					fs::ActixFileLoader::load(
						"/public",
						&client_web_cfg.public_dir,
					)
					.use_last_modified(true),
				)
				.service(routes::index_view)
		})
		.bind(addr)?
		.run()
		.await
	}

	async fn forward(
		req: HttpRequest,
		payload: web::Payload,
		url: web::Data<Url>,
		client: Data<Client>,
	) -> Result<HttpResponse, Error> {
		let mut url_ = url.get_ref().clone();
		url_.set_path(req.uri().path());
		url_.set_query(req.uri().query());

		let forwarded_req = client
			.request_from(url_.as_str(), req.head())
			.no_decompress();

		let forwarded_req = match req.head().peer_addr {
			| Some(addr) => forwarded_req
				.insert_header(("x-forwarded-for", addr.ip().to_string())),
			| None => forwarded_req,
		};

		let res = forwarded_req
			.send_stream(payload)
			.await
			.map_err(error::ErrorInternalServerError)?;

		let mut client_resp = HttpResponse::build(res.status());

		for (hname, hvalue) in res
			.headers()
			.iter()
			.filter(|(hname, _)| *hname != "connection")
		{
			client_resp.insert_header((hname.clone(), hvalue.clone()));
		}

		Ok(client_resp.streaming(res))
	}
}
