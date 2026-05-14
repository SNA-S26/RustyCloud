use axum::{
    Form,
    body::Body,
    extract::{Multipart, Query},
    http::{HeaderMap, header},
    response::{Html, IntoResponse, Redirect},
};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

use crate::{
    handlers::require_auth::require_auth,
    services::files::{get_files, remove_file, write_file},
};

/*
 * Handles GET /dashboard
 */
pub async fn serve_dashboard(jar: CookieJar) -> (CookieJar, impl IntoResponse) {
    // Check the authenticity
    let username = match require_auth(jar.clone()).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    let files = get_files(username.clone()).await;

    let mut ctx = tera::Context::new();
    ctx.insert("files", &files);
    ctx.insert("username", &username);

    let html = tera::Tera::new("templates/**/*.html")
        .unwrap()
        .render("dashboard.html", &ctx)
        .unwrap();

    (jar, Html(html).into_response())
}

/*
 * Handles POST /upload-file
 */
pub async fn upload_file(
    jar: CookieJar,
    mut multipart: Multipart,
) -> (CookieJar, impl IntoResponse) {
    // Check the authenticity
    let username = match require_auth(jar.clone()).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    // Write files
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        write_file(username.clone(), filename, data).await;
    }

    (jar, Redirect::to("/dashboard").into_response())
}

#[derive(Deserialize)]
pub struct DeleteFileForm {
    filename: String,
}

/*
 * Handles POST /delete-file
 */
pub async fn delete_file(
    jar: CookieJar,
    Form(form): Form<DeleteFileForm>,
) -> (CookieJar, impl IntoResponse) {
    // Check the authenticity
    let username = match require_auth(jar.clone()).await {
        Ok(v) => v,
        Err(e) => return e,
    };

    remove_file(username, form.filename).await;

    (jar, Redirect::to("/dashboard").into_response())
}

#[derive(Deserialize)]
pub struct FileQuery {
    filename: String,
}

/*
 * Handles GET /file
 */
pub async fn get_file(jar: CookieJar, Query(params): Query<FileQuery>) -> impl IntoResponse {
    // Check the authenticity
    let username = match require_auth(jar.clone()).await {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    // Get the file
    let filename = params.filename;
    let data = crate::services::files::get_file(&username, &filename).await;
    let data = match data {
        Ok(v) => v,
        Err(_) => vec![],
    };

    // Send the file
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&format!("attachment; filename = \"{}\"", filename))
            .unwrap_or(header::HeaderValue::from_static("file")),
    );

    (headers, Body::from(data)).into_response()
}
