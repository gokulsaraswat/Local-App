# Patch 4 Notes

## Summary

Patch 4 introduces the local inventory ledger core.

## Highlights

- adds `inventory_stock_movements`
- backfills opening balances for existing stock quantities
- adds inventory workspace page and navigation
- records stock movements locally without cloud dependencies
- includes inventory movement data in exports and import previews
- keeps catalog stock edits synchronized into the ledger

## Upgrade impact

- migration required
- no destructive schema changes to existing Patch 3 tables
- existing stock quantities remain intact and gain movement history
