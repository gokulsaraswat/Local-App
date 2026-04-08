# MIGRATION_NOTES

## Migration file
- `src-tauri/src/core/migrations/003_catalog_core.sql`

## Schema version change
- previous schema version: `2`
- new schema version: `3`

## Migration actions
1. add compatibility columns:
   - `tax_profiles.prices_include_tax`
   - `receipt_profiles.show_email`
   - `receipt_profiles.show_business_code`
   - `sequence_counters.reset_policy`
2. backfill defaults for those new columns
3. create catalog tables:
   - `catalog_categories`
   - `catalog_units`
   - `catalog_items`
   - `catalog_item_barcodes`
4. create business-scoped indexes for category codes, item SKUs, and primary barcodes

## Seed changes after migration
On bootstrap, Patch 3 also ensures:
- system units exist
- the demo business gets a starter catalog if demo data is enabled

## Downgrade note
No automatic reverse migration is supplied. Use a pre-patch database backup if rollback is needed.
