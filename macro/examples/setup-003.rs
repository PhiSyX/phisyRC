/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

// Cet exemple montre l'utilisation du générique Async.

// Exemple provenant de https://github.com/actix/actix-web/blob/master/actix-web/examples/on-connect.rs

use std::{any::Any, io, net::SocketAddr};

use actix_web::{
	dev::Extensions, rt::net::TcpStream, web, App, HttpRequest, HttpResponse,
	HttpServer, Responder,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ConnectionInfo {
	bind: SocketAddr,
	peer: SocketAddr,
	ttl: Option<u32>,
}

async fn route_whoami(req: HttpRequest) -> impl Responder {
	match req.conn_data::<ConnectionInfo>() {
		| Some(info) => HttpResponse::Ok().body(format!(
			"Here is some info about your connection:\n\n{:#?}",
			info
		)),
		| None => HttpResponse::InternalServerError()
			.body("Missing expected request extension data"),
	}
}

fn get_conn_info(connection: &dyn Any, data: &mut Extensions) {
	if let Some(sock) = connection.downcast_ref::<TcpStream>() {
		data.insert(ConnectionInfo {
			bind: sock.local_addr().unwrap(),
			peer: sock.peer_addr().unwrap(),
			ttl: sock.ttl().ok(),
		});
	} else {
		unreachable!(
			"connection should only be plaintext since no TLS is set up"
		);
	}
}

#[phisyrc_macro::setup]
async fn main<Async>() -> io::Result<()>
where
	Async: actix_web,
{
	let bind = ("127.0.0.1", 9999);

	println!("Listening on {bind:?}");

	HttpServer::new(|| App::new().default_service(web::to(route_whoami)))
		.on_connect(get_conn_info)
		.bind(bind)?
		.workers(1)
		.run()
		.await
}
