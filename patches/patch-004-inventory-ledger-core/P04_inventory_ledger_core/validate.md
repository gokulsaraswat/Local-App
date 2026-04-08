# Validation Checklist - Patch 4

- [ ] Apply Patch 4 on top of a Patch 3 project
- [ ] `npm install` completes
- [ ] `npm run validate:patch4` passes
- [ ] `npm run tauri dev` opens the desktop shell
- [ ] Inventory appears in the left navigation
- [ ] Dashboard shows inventory summary cards
- [ ] Inventory page lists stock items from the catalog
- [ ] Recording stock in/out creates movement history and updates quantity
- [ ] Editing reorder level persists locally
- [ ] Catalog stock edits create inventory sync movement rows
- [ ] Export bundle includes `inventoryMovements`
- [ ] Import preview shows `movementCount`
