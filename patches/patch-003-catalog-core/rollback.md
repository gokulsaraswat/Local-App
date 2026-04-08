# Rollback notes

## File rollback
The patch applier stores overwritten files in:

```text
.patch-backups/P003_catalog_core/
```

You can restore those files manually if needed.

## Database rollback
Patch 3 includes schema migration 003. Once the app starts after patch apply, the local database may be upgraded to schema version 3.

Recommended rollback path:
1. close the app
2. restore the pre-patch SQLite snapshot from your backup folder
3. restore overwritten source files from `.patch-backups/P003_catalog_core/`
4. reinstall dependencies if required

Do not attempt an in-place SQL downgrade unless you fully control the target database and have tested the downgrade separately.
