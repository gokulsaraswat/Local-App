# Migration Notes - Patch 2

Patch 2 adds new tables only and does not drop or rename Patch 1 tables.

## New tables
- `tax_profiles`
- `receipt_profiles`
- `sequence_counters`

## Backfill behavior on first app start after applying Patch 2
For every existing business row:
- create one default tax profile if missing
- create one default receipt profile if missing
- seed sequence counters for `sale`, `purchase`, `customer`, and `supplier` if missing
- preserve the current `active_business_id` where possible

## Compatibility
Patch 2 expects a project already initialized from Patch 1.
