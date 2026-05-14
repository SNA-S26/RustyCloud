use axum::response::Html;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

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

// Extracts the cookie if present
pub fn extract_cookie(jar: CookieJar, name: &str) -> Option<String> {
    jar.get(name).map(|c| c.value().to_string())
}

// Removes the cookie if present
pub fn remove_cookie(jar: CookieJar, name: &str) -> CookieJar {
    jar.remove(Cookie::from(name.to_string()))
}

pub fn extract_credentials(jar: CookieJar) -> (Option<String>, Option<String>) {
    let username = extract_cookie(jar.clone(), "username");
    let password = extract_cookie(jar, "password");
    (username, password)
}

pub fn remove_credentials(jar: CookieJar) -> CookieJar {
    let jar = remove_cookie(jar, "username");
    remove_cookie(jar, "password")
}

pub fn add_credentials(jar: CookieJar, username: String, password: String) -> CookieJar {
    let username_cookie = Cookie::build(("username", username))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();
    let password_cookie = Cookie::build(("password", password))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();

    jar.add(username_cookie).add(password_cookie)
}
