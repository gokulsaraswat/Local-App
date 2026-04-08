# Rollback Notes

## Safe rollback options
1. Restore from version control if the target project is tracked.
2. Restore files from `.patch-backups/P002_multi_business_workspace_settings_core/` created by the patch applier.
3. Restore the SQLite database from a pre-patch backup snapshot if runtime data must also be reverted.

## Notes
- Patch 2 introduces new tables only; it does not remove Patch 1 tables.
- Source rollback does not automatically delete rows created in the local database.
- If you need both code and data rollback, restore both the source tree and the SQLite file.
