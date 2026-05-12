use axum::response::Html;
use axum_extra::extract::cookie::{Cookie, CookieJar};

/*
 * Renders error text using tera
 *
 * Arguments:
 * - path:  Path to the HTML template
 * - err:   Error description
 */
pub async fn render_error(path: &str, err: Option<String>) -> Html<String> {
    let mut ctx = tera::Context::new();

    if let Some(msg) = err {
        ctx.insert("error", &msg);
    }

    let html = tera::Tera::new("templates/**/*.html")
        .unwrap()
        .render(path, &ctx)
        .unwrap();

    Html(html)
}

/*
 * Extracts cookie if present
 * and removes it from jar
 */
pub fn extract_cookie(jar: CookieJar, name: &str) -> (CookieJar, Option<String>) {
    let value = jar.get(name).map(|c| c.value().to_string());

    let jar = jar.remove(Cookie::from(name.to_string()));
    (jar, value)
}
