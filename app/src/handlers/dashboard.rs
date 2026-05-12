use axum::response::{Html, IntoResponse};
use axum_extra::extract::cookie::CookieJar;

use crate::services::files::{delete_file, get_files, write_file};

pub async fn serve_dashboard(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    // TODO: Session -> username, redirect to / (invalid session)

    let files = get_files("STUB".to_string()).await;

    let mut ctx = tera::Context::new();
    ctx.insert("files", &files);

    let html = tera::Tera::new("templates/**/*.html")
        .unwrap()
        .render("dashboard.html", &ctx)
        .unwrap();

    (jar, Html(html))
}
