# Migration Notes — Patch 1

## Database migration impact
This is the initial schema bootstrap.

## Applied schema artifacts
- `001_base.sql`

## Existing data impact
- None expected when applying to an empty/new project.
- If forcing over an existing directory, no automatic merge is performed.

## First-run initialization
On first application launch, the backend:
1. resolves local app directories
2. creates required folders
3. creates the SQLite database
4. applies the base migration
5. records patch metadata
6. seeds demo business/settings/activity if no business exists
