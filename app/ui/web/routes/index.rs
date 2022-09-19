use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index_view(templates: web::Data<tera::Tera>) -> impl Responder {
	let ctx = tera::Context::new();

	let maybe_view = templates.render("index.html", &ctx).map(|content| {
		HttpResponse::Ok().content_type("text/html").body(content)
	});

	if maybe_view.is_err() {
		return templates
			.render("errors/error500.html", &ctx)
			.map(|err| {
				HttpResponse::InternalServerError()
					.content_type("text/html")
					.body(err)
			})
			.unwrap();
	}

	maybe_view.unwrap()
}
