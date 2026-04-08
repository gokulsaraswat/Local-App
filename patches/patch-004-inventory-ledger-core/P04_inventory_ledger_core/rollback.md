# Rollback Notes - Patch 4

## File rollback

Restore overwritten files from:

```text
.patch-backups/P004_inventory_ledger_core/
```

## Database rollback

If the live database has already run migration 004, the safest rollback is:

1. close the app
2. restore the database from a backup created before Patch 4
3. restore the overwritten project files from `.patch-backups/P004_inventory_ledger_core/`

## Notes

Manual rollback of migration 004 is not recommended on live business data because movement history may already have been written after the patch was applied.
