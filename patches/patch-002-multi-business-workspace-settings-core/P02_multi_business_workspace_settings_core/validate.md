# Validation Checklist

## Structural checks completed in this bundle
- [x] `apply_patch.mjs` syntax check passed
- [x] patch manifest JSON parsed successfully
- [x] Tauri config JSON parsed successfully
- [x] `node scripts/validate-patch2.mjs` passed against the patched project tree
- [x] TypeScript structural check passed with local validation stubs
- [ ] Full Rust compile was not executed in this container because the Rust toolchain is unavailable here
- [ ] Full `npm install` and desktop launch were not executed in this container

## Manual validation after apply
- [ ] run `npm install`
- [ ] run `npm run validate:patch2`
- [ ] run `npm run tauri dev`
- [ ] verify dashboard shows Patch 2 workspace messaging
- [ ] create a second business workspace
- [ ] switch active business and confirm settings change with the active workspace
- [ ] archive a non-last active business
- [ ] save default tax profile
- [ ] save default receipt profile
- [ ] save sequence counters
- [ ] export a workspace snapshot and confirm the JSON includes `taxProfiles`, `receiptProfiles`, and `sequenceCounters`
