use tauri::AppHandle;

use crate::{
    core::{db, error::CommandResult, inventory},
    domain::models::{
        InventoryMovement, InventoryStockItem, InventoryWorkspace, SaveInventoryMovementInput,
        SaveInventoryStockRuleInput,
    },
};

#[tauri::command]
pub fn load_inventory_workspace(app: AppHandle) -> CommandResult<InventoryWorkspace> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        inventory::load_inventory_workspace(conn, &active_business.id)
    })
}

#[tauri::command]
pub fn record_inventory_movement(
    app: AppHandle,
    input: SaveInventoryMovementInput,
) -> CommandResult<InventoryMovement> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        inventory::record_inventory_movement(conn, &active_business.id, &input)
    })
}

#[tauri::command]
pub fn save_inventory_stock_rule(
    app: AppHandle,
    input: SaveInventoryStockRuleInput,
) -> CommandResult<InventoryStockItem> {
    db::with_connection(&app, |conn, _paths| {
        let active_business = db::get_active_business(conn)?;
        inventory::save_inventory_stock_rule(conn, &active_business.id, &input)
    })
}
