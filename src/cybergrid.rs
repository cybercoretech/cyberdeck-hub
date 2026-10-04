//! CYBERGRID bridge to the shared Cybercore theme engine.

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

fn catalog() -> Result<cybercore::theme::ThemeCatalog, cybercore::theme::ThemeError> {
    cybercore::theme::ThemeCatalog::load()
}

/// Group names come from embedded theme metadata, not slug guesses or a
/// checkout-specific read of the Cybercore source tree.
pub fn theme_families() -> Vec<(String, Vec<String>)> {
    let Ok(catalog) = catalog() else {
        return Vec::new();
    };
    let mut families: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for (id, entry) in catalog.iter() {
        if !entry.builtin {
            continue;
        }
        families
            .entry(entry.document.metadata.family.clone())
            .or_default()
            .push(id.to_string());
    }
    families.into_iter().collect()
}

fn palette_json(palette: &cybercore::schema::Palette) -> Value {
    json!({
        "bg": palette.bg,
        "white": palette.white,
        "acid_green": palette.acid_green,
        "hot_pink": palette.hot_pink,
        "purple": palette.purple,
        "cyan": palette.cyan,
        "orange": palette.orange,
        "red": palette.red,
        "panel": palette.panel,
        "line": palette.line,
        "muted": palette.muted,
    })
}

fn error(status: StatusCode, message: impl ToString) -> Response {
    (status, Json(json!({ "error": message.to_string() }))).into_response()
}

pub async fn list_themes() -> Response {
    let Ok(catalog) = catalog() else {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "could not load theme catalog",
        );
    };
    let themes: Vec<Value> = catalog
        .iter()
        .map(|(id, entry)| {
            let document = &entry.document;
            json!({
                "id": id,
                "name": document.metadata.name,
                "family": document.metadata.family,
                "description": document.metadata.description,
                "author": document.metadata.author,
                "builtin": entry.builtin,
                "palette": palette_json(&document.palette),
                "variants": document.variants,
                "design": document.design,
            })
        })
        .collect();
    Json(json!({
        "active": catalog.active_id(),
        "appearance": catalog.active_appearance(),
        "themes": themes
    }))
    .into_response()
}

#[derive(serde::Deserialize)]
pub struct CssQuery {
    appearance: Option<cybercore::theme::Appearance>,
}

pub async fn theme_css(Path(id): Path<String>, query: axum::extract::Query<CssQuery>) -> Response {
    let Ok(catalog) = catalog() else {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "could not load theme catalog",
        );
    };
    let Some(entry) = catalog.get(&id) else {
        return error(StatusCode::NOT_FOUND, "theme not found");
    };
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        entry
            .document
            .to_css(query.appearance.unwrap_or(catalog.active_appearance())),
    )
        .into_response()
}

pub async fn select_theme(Path(id): Path<String>) -> Response {
    let mut catalog = match catalog() {
        Ok(catalog) => catalog,
        Err(error) => return self::error(StatusCode::BAD_REQUEST, error),
    };
    match catalog.select(&id) {
        Ok(()) => Json(json!({ "active": id })).into_response(),
        Err(error) => self::error(StatusCode::BAD_REQUEST, error),
    }
}

pub async fn select_appearance(Path(appearance): Path<String>) -> Response {
    let appearance = match appearance.as_str() {
        "dark" => cybercore::theme::Appearance::Dark,
        "light" => cybercore::theme::Appearance::Light,
        _ => return error(StatusCode::BAD_REQUEST, "appearance must be dark or light"),
    };
    let mut catalog = match catalog() {
        Ok(catalog) => catalog,
        Err(error) => return self::error(StatusCode::BAD_REQUEST, error),
    };
    match catalog.set_appearance(appearance) {
        Ok(()) => Json(json!({ "appearance": appearance })).into_response(),
        Err(error) => self::error(StatusCode::BAD_REQUEST, error),
    }
}
