# Validation checklist

Run after applying the patch from the target project root:

```bash
npm install
npm run validate:patch3
npm run tauri dev
```

Manual checks:
- [ ] App starts without bootstrap errors
- [ ] Sidebar shows Catalog navigation
- [ ] Dashboard hero reflects Patch 3 catalog core
- [ ] Catalog page loads categories, units, taxes, and items for the active business
- [ ] Demo workspace contains seed items such as beverages / retail / service entries
- [ ] Creating a category saves and appears immediately after refresh
- [ ] Creating a unit saves and is selectable in the item form
- [ ] Creating an item updates catalog summary counts
- [ ] Archiving an item removes it from the default active list
- [ ] Export bundle now includes `catalogCategories`, `catalogUnits`, `catalogItems`, and `catalogBarcodes`
- [ ] Import preview reports business, category, and item counts
