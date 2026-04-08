# Patch 2 Notes

## Goal
Turn the Patch 1 desktop foundation into a true multi-business local workspace.

## Adds
- Multi-business creation flow
- Safe active business switching
- Soft archive support for businesses
- Active business dashboard and navigation updates
- Per-business default tax profile foundation
- Per-business default receipt profile foundation
- Per-business sequence counters foundation
- Expanded export bundle scope for workspace metadata
- Validation script for Patch 2 structure

## Data changes
- Added `tax_profiles`
- Added `receipt_profiles`
- Added `sequence_counters`
- Seed/backfill now ensures every business has default workspace support rows
- Patch level moves from `P001_foundation_base_structure` to `P002_multi_business_workspace_settings_core`

## Deferred
- Product catalog
- POS billing flows
- Inventory ledgers
- Full import apply
- Local users and roles
