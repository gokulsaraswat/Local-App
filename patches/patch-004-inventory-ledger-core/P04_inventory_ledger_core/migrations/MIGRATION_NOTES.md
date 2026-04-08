# Migration Notes - Patch 4

Schema version moves from `3` to `4`.

## Migration 004

Creates `inventory_stock_movements` and supporting indexes.

## Backfill

Existing tracked stock items with `stock_quantity > 0` receive an `opening_balance` movement row using deterministic IDs:

- `opening-balance:<item_id>`

## Fresh installs

On a brand-new workspace, migration 004 runs before demo seed data exists. After seeding, startup backfill inserts opening-balance rows for seeded stock items when needed.

## Operational recommendation

Create a local backup before opening a live database with Patch 4 for the first time.
