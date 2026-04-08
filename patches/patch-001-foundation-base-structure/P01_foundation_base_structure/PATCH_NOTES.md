# Patch Notes — P01 Foundation Base Structure

## Added
- Tauri desktop shell
- React frontend shell
- SQLite initialization and migrations
- Patch history registration
- Business profile management
- Settings management
- Dashboard shell
- Data Center shell
- Backup snapshot command
- Export foundation snapshot command
- Import preview command
- Demo seed data

## Introduced module boundaries
### Frontend
- `src/app` for runtime state/bootstrap
- `src/modules/*` for feature shells
- `src/shared/*` for types, API bridge, shared helpers

### Backend
- `src-tauri/src/commands` for Tauri command surface
- `src-tauri/src/core` for DB/migrations/paths/patching/bootstrap internals
- `src-tauri/src/domain` for serializable business-facing models and dashboard composition

## Future extension points intentionally left open
- product catalog module
- inventory ledger module
- POS transaction module
- reporting/query module
- import/apply migrations
- backup scheduling
- multi-user access layer
- restaurant/retail workflow packs

## Compatibility
- Base project only
- No prior patch required
