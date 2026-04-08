# Rollback Notes — Patch 1

## If the patch was applied to an empty project
Delete the target project directory.

## If the patch was applied with `--force`
Restore files from:
```text
.patch-backups/P001_foundation_base_structure/
```

## If you also want to remove local seeded data
Delete the app data directory created by Tauri for the app identifier:
```text
com.localfirst.businessmanager
```

That removes:
- the SQLite database
- export files
- backup files
- runtime logs
