use tauri::AppHandle;

use crate::{
    core::{catalog, db, error::CommandResult},
    domain::models::{
        CatalogCategory, CatalogItem, CatalogUnit, CatalogWorkspace, SaveCatalogCategoryInput,
        SaveCatalogItemInput, SaveCatalogUnitInput,
    },
};

#[tauri::command]
pub fn load_catalog_workspace(app: AppHandle) -> CommandResult<CatalogWorkspace> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::load_catalog_workspace(conn, &active_business.id)
    })
}

#[tauri::command]
pub fn save_catalog_category(
    app: AppHandle,
    input: SaveCatalogCategoryInput,
) -> CommandResult<CatalogCategory> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_category(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn save_catalog_unit(
    app: AppHandle,
    input: SaveCatalogUnitInput,
) -> CommandResult<CatalogUnit> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_unit(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn save_catalog_item(
    app: AppHandle,
    input: SaveCatalogItemInput,
) -> CommandResult<CatalogItem> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::save_catalog_item(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn set_catalog_item_archived(
    app: AppHandle,
    item_id: String,
    archived: bool,
) -> CommandResult<CatalogItem> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        catalog::set_catalog_item_archived(conn, &active_business.id, &item_id, archived)
    })
}
