import { useCallback, useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import {
  loadCatalogWorkspace,
  saveCatalogCategory,
  saveCatalogItem,
  saveCatalogUnit,
  setCatalogItemArchived
} from "../../shared/api";
import type {
  CatalogItemView,
  CatalogWorkspace,
  SaveCatalogCategoryInput,
  SaveCatalogItemInput,
  SaveCatalogUnitInput
} from "../../shared/types";
import {
  formatCurrency,
  formatDateTime,
  linesFromMultilineValue,
  multilineValueFromLines,
  titleCaseWords
} from "../../shared/utils";

const emptyCategoryForm: SaveCatalogCategoryInput = {
  id: undefined,
  name: "",
  code: "",
  parentId: null,
  itemScope: "all",
  sortOrder: 10,
  notes: null
};

const emptyUnitForm: SaveCatalogUnitInput = {
  id: undefined,
  name: "",
  code: "",
  symbol: "",
  allowFractional: false
};

const emptyItemForm: SaveCatalogItemInput = {
  id: undefined,
  categoryId: null,
  unitId: null,
  taxProfileId: null,
  itemKind: "stock",
  name: "",
  displayName: null,
  sku: null,
  barcodes: [],
  description: null,
  sellingPrice: 0,
  costPrice: 0,
  trackStock: true,
  stockQuantity: 0,
  reorderLevel: 0,
  imagePath: null,
  isActive: true
};

function StockIndicator({ item }: { item: CatalogItemView["item"] }) {
  if (!item.trackStock || item.archivedAt) return <span className="pill neutral">No stock tracking</span>;
  if (item.reorderLevel > 0 && item.stockQuantity <= item.reorderLevel) {
    return <span className="pill warning">Low stock</span>;
  }
  return <span className="pill success">Stock okay</span>;
}

export function CatalogPage() {
  const { data, refresh } = useAppState();
  const [workspace, setWorkspace] = useState<CatalogWorkspace | null>(null);
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [search, setSearch] = useState("");
  const [kindFilter, setKindFilter] = useState("all");
  const [showArchived, setShowArchived] = useState(false);
  const [categoryForm, setCategoryForm] =
    useState<SaveCatalogCategoryInput>(emptyCategoryForm);
  const [unitForm, setUnitForm] = useState<SaveCatalogUnitInput>(emptyUnitForm);
  const [itemForm, setItemForm] = useState<SaveCatalogItemInput>(emptyItemForm);
  const [barcodeText, setBarcodeText] = useState("");

  const loadWorkspace = useCallback(async () => {
    setLoading(true);
    try {
      const next = await loadCatalogWorkspace();
      setWorkspace(next);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadWorkspace();
  }, [loadWorkspace, data?.activeBusiness.id]);

  const filteredItems = useMemo(() => {
    if (!workspace) return [];
    const query = search.trim().toLowerCase();
    return workspace.items.filter((entry) => {
      const matchesArchived = showArchived
        ? true
        : entry.item.archivedAt === null;
      const matchesKind = kindFilter === "all" || entry.item.itemKind === kindFilter;
      const haystack = [
        entry.item.name,
        entry.item.displayName,
        entry.item.sku,
        entry.item.primaryBarcode,
        entry.categoryName,
        entry.taxLabel,
        entry.unitCode
      ]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();

      const matchesQuery = !query || haystack.includes(query);
      return matchesArchived && matchesKind && matchesQuery;
    });
  }, [kindFilter, search, showArchived, workspace]);

  if (!data) return null;

  function resetCategoryForm() {
    setCategoryForm(emptyCategoryForm);
  }

  function resetUnitForm() {
    setUnitForm(emptyUnitForm);
  }

  function resetItemForm() {
    const currentData = data;
    if (!currentData) {
      setItemForm(emptyItemForm);
      setBarcodeText("");
      return;
    }
    setItemForm({
      ...emptyItemForm,
      taxProfileId: currentData.activeTaxProfile.id
    });
    setBarcodeText("");
  }

  function editCategory(categoryId: string) {
    const selected = workspace?.categories.find((category) => category.id === categoryId);
    if (!selected) return;
    setCategoryForm({
      id: selected.id,
      name: selected.name,
      code: selected.code,
      parentId: selected.parentId,
      itemScope: selected.itemScope,
      sortOrder: selected.sortOrder,
      notes: selected.notes
    });
  }

  function editUnit(unitId: string) {
    const selected = workspace?.units.find((unit) => unit.id === unitId);
    if (!selected || selected.isSystem) return;
    setUnitForm({
      id: selected.id,
      name: selected.name,
      code: selected.code,
      symbol: selected.symbol,
      allowFractional: selected.allowFractional
    });
  }

  function editItem(entry: CatalogItemView) {
    setItemForm({
      id: entry.item.id,
      categoryId: entry.item.categoryId,
      unitId: entry.item.unitId,
      taxProfileId: entry.item.taxProfileId,
      itemKind: entry.item.itemKind,
      name: entry.item.name,
      displayName: entry.item.displayName,
      sku: entry.item.sku,
      barcodes: entry.barcodes.map((barcode) => barcode.barcode),
      description: entry.item.description,
      sellingPrice: entry.item.sellingPrice,
      costPrice: entry.item.costPrice,
      trackStock: entry.item.trackStock,
      stockQuantity: entry.item.stockQuantity,
      reorderLevel: entry.item.reorderLevel,
      imagePath: entry.item.imagePath,
      isActive: entry.item.isActive
    });
    setBarcodeText(multilineValueFromLines(entry.barcodes.map((barcode) => barcode.barcode)));
  }

  async function handleCategorySave() {
    setStatusMessage("Saving category…");
    try {
      await saveCatalogCategory({
        ...categoryForm,
        code: categoryForm.code.toUpperCase(),
        notes: categoryForm.notes?.trim() ? categoryForm.notes : null
      });
      resetCategoryForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog category saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save category."
      );
    }
  }

  async function handleUnitSave() {
    setStatusMessage("Saving unit…");
    try {
      await saveCatalogUnit({
        ...unitForm,
        code: unitForm.code.toUpperCase(),
        symbol: unitForm.symbol.toUpperCase()
      });
      resetUnitForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog unit saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save unit."
      );
    }
  }

  async function handleItemSave() {
    setStatusMessage("Saving item…");
    try {
      await saveCatalogItem({
        ...itemForm,
        sku: itemForm.sku?.trim() ? itemForm.sku : null,
        barcodes: linesFromMultilineValue(barcodeText),
        description: itemForm.description?.trim() ? itemForm.description : null,
        imagePath: itemForm.imagePath?.trim() ? itemForm.imagePath : null,
        displayName: itemForm.displayName?.trim() ? itemForm.displayName : null
      });
      resetItemForm();
      await loadWorkspace();
      await refresh();
      setStatusMessage("Catalog item saved locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save item."
      );
    }
  }

  async function toggleArchive(entry: CatalogItemView) {
    setStatusMessage(
      entry.item.archivedAt ? "Restoring item…" : "Archiving item…"
    );
    try {
      await setCatalogItemArchived(entry.item.id, !entry.item.archivedAt);
      await loadWorkspace();
      await refresh();
      setStatusMessage(
        entry.item.archivedAt
          ? "Catalog item restored locally."
          : "Catalog item archived locally."
      );
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to update item state."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Catalog core + Patch 4 inventory sync</div>
          <h2>{data.activeBusiness.name} item master</h2>
          <p>
            Build and maintain a clean local catalog for retail items, menu items, and
            services. Patch 4 now records stock quantity changes into the inventory
            ledger so catalog edits and inventory history stay aligned locally.
          </p>
        </div>
        <div className="hero-actions align-start">
          <span className="meta-chip">Categories: {workspace?.summary.categoryCount ?? 0}</span>
          <span className="meta-chip">Active items: {workspace?.summary.activeItems ?? 0}</span>
          <span className="meta-chip">Low stock: {workspace?.summary.lowStockCandidates ?? 0}</span>
        </div>
      </section>

      <section className="card-grid card-grid-4">
        <article className="card">
          <div className="card-label">Menu items</div>
          <div className="kpi-value">{workspace?.summary.menuItemCount ?? 0}</div>
          <p className="card-note">Prepared or food-facing items without full recipe deduction yet.</p>
        </article>
        <article className="card">
          <div className="card-label">Stock items</div>
          <div className="kpi-value">{workspace?.summary.stockItemCount ?? 0}</div>
          <p className="card-note">Tracked products with current stock and reorder thresholds.</p>
        </article>
        <article className="card">
          <div className="card-label">Service items</div>
          <div className="kpi-value">{workspace?.summary.serviceItemCount ?? 0}</div>
          <p className="card-note">Simple non-stock services that can be billed in later patches.</p>
        </article>
        <article className="card">
          <div className="card-label">Archived items</div>
          <div className="kpi-value">{workspace?.summary.archivedItems ?? 0}</div>
          <p className="card-note">Archived records stay local and restorable for safer operations.</p>
        </article>
      </section>

      <section className="split-grid catalog-primary-grid">
        <article className="card">
          <div className="card-header">
            <h2>Item catalog</h2>
            <span className="pill success">{filteredItems.length} shown</span>
          </div>

          <div className="catalog-filter-bar">
            <input
              placeholder="Search name, SKU, barcode, category…"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
            />
            <select
              value={kindFilter}
              onChange={(event) => setKindFilter(event.target.value)}
            >
              <option value="all">All types</option>
              <option value="stock">Stock</option>
              <option value="menu">Menu</option>
              <option value="service">Service</option>
            </select>
            <label className="toggle-row inline-toggle">
              <input
                type="checkbox"
                checked={showArchived}
                onChange={(event) => setShowArchived(event.target.checked)}
              />
              <span>Show archived</span>
            </label>
          </div>

          <div className="status-banner">
            {loading ? "Loading catalog workspace…" : statusMessage || "Catalog ready."}
          </div>

          <div className="catalog-list">
            {filteredItems.length > 0 ? (
              filteredItems.map((entry) => (
                <article className="catalog-item-card" key={entry.item.id}>
                  <div className="card-header compact-card-header">
                    <div>
                      <h3>{entry.item.displayName || entry.item.name}</h3>
                      <div className="muted-text">
                        {titleCaseWords(entry.item.itemKind)}
                        {entry.categoryName ? ` · ${entry.categoryName}` : ""}
                        {entry.unitCode ? ` · ${entry.unitCode}` : ""}
                      </div>
                    </div>
                    <div className="tag-list compact-tags">
                      <StockIndicator item={entry.item} />
                      <span className={`pill ${entry.item.isActive ? "success" : "neutral"}`}>
                        {entry.item.isActive ? "Active" : "Inactive"}
                      </span>
                    </div>
                  </div>

                  <div className="detail-list compact-detail-list">
                    <div>
                      <span>Selling price</span>
                      <code>
                        {formatCurrency(entry.item.sellingPrice, data.activeBusiness.currencyCode)}
                      </code>
                    </div>
                    <div>
                      <span>Cost price</span>
                      <code>
                        {formatCurrency(entry.item.costPrice, data.activeBusiness.currencyCode)}
                      </code>
                    </div>
                    <div>
                      <span>SKU</span>
                      <code>{entry.item.sku || "—"}</code>
                    </div>
                    <div>
                      <span>Barcode</span>
                      <code>{entry.item.primaryBarcode || "—"}</code>
                    </div>
                    <div>
                      <span>Stock</span>
                      <code>
                        {entry.item.trackStock ? entry.item.stockQuantity : "Not tracked"}
                      </code>
                    </div>
                    <div>
                      <span>Updated</span>
                      <code>{formatDateTime(entry.item.updatedAt)}</code>
                    </div>
                  </div>

                  <div className="catalog-item-description muted-text">
                    {entry.item.description || "No description saved yet."}
                  </div>

                  <div className="inline-actions">
                    <button
                      className="secondary-button"
                      type="button"
                      onClick={() => editItem(entry)}
                    >
                      Edit item
                    </button>
                    <button
                      className="secondary-button"
                      type="button"
                      onClick={() => void toggleArchive(entry)}
                    >
                      {entry.item.archivedAt ? "Restore" : "Archive"}
                    </button>
                  </div>
                </article>
              ))
            ) : (
              <p className="muted-text">No items match the current filter yet.</p>
            )}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>{itemForm.id ? "Edit item" : "Add catalog item"}</h2>
            <span className="pill warning">Local only</span>
          </div>

          <div className="form-grid">
            <label>
              <span>Item type</span>
              <select
                value={itemForm.itemKind}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    itemKind: event.target.value,
                    trackStock: event.target.value === "stock" ? current.trackStock : false
                  }))
                }
              >
                <option value="stock">Stock / retail item</option>
                <option value="menu">Menu item</option>
                <option value="service">Service item</option>
              </select>
            </label>

            <label>
              <span>Name</span>
              <input
                value={itemForm.name}
                onChange={(event) =>
                  setItemForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>

            <label>
              <span>Display name</span>
              <input
                value={itemForm.displayName ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    displayName: event.target.value
                  }))
                }
              />
            </label>

            <label>
              <span>SKU</span>
              <input
                value={itemForm.sku ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({ ...current, sku: event.target.value }))
                }
              />
            </label>

            <label>
              <span>Category</span>
              <select
                value={itemForm.categoryId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    categoryId: event.target.value || null
                  }))
                }
              >
                <option value="">No category</option>
                {workspace?.categories
                  .filter((category) => !category.archivedAt)
                  .map((category) => (
                    <option key={category.id} value={category.id}>
                      {category.name}
                    </option>
                  ))}
              </select>
            </label>

            <label>
              <span>Unit</span>
              <select
                value={itemForm.unitId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    unitId: event.target.value || null
                  }))
                }
              >
                <option value="">No unit</option>
                {workspace?.units.map((unit) => (
                  <option key={unit.id} value={unit.id}>
                    {unit.name} ({unit.code})
                  </option>
                ))}
              </select>
            </label>

            <label>
              <span>Tax profile</span>
              <select
                value={itemForm.taxProfileId ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    taxProfileId: event.target.value || null
                  }))
                }
              >
                <option value="">No tax profile</option>
                {workspace?.taxProfiles.map((profile) => (
                  <option key={profile.id} value={profile.id}>
                    {profile.name} · {profile.taxLabel} · {profile.defaultRate}%
                  </option>
                ))}
              </select>
            </label>

            <label>
              <span>Selling price</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.sellingPrice}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    sellingPrice: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label>
              <span>Cost price</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.costPrice}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    costPrice: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={itemForm.trackStock}
                disabled={itemForm.itemKind !== "stock"}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    trackStock: event.target.checked
                  }))
                }
              />
              <span>Track stock quantity</span>
            </label>

            <label className="toggle-row">
              <input
                type="checkbox"
                checked={itemForm.isActive}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    isActive: event.target.checked
                  }))
                }
              />
              <span>Item is active</span>
            </label>

            <label>
              <span>Current stock</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.stockQuantity}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    stockQuantity: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label>
              <span>Reorder level</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={itemForm.reorderLevel}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    reorderLevel: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Barcodes (one per line)</span>
              <textarea
                rows={4}
                value={barcodeText}
                onChange={(event) => setBarcodeText(event.target.value)}
              />
            </label>

            <label className="form-span-2">
              <span>Image path</span>
              <input
                value={itemForm.imagePath ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    imagePath: event.target.value
                  }))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Description</span>
              <textarea
                rows={4}
                value={itemForm.description ?? ""}
                onChange={(event) =>
                  setItemForm((current) => ({
                    ...current,
                    description: event.target.value
                  }))
                }
              />
            </label>
          </div>

          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleItemSave()}>
              {itemForm.id ? "Save item changes" : "Add catalog item"}
            </button>
            <button className="secondary-button" type="button" onClick={resetItemForm}>
              Clear form
            </button>
          </div>
        </article>
      </section>

      <section className="split-grid catalog-secondary-grid">
        <article className="card">
          <div className="card-header">
            <h2>Categories</h2>
            <span className="pill neutral">{workspace?.categories.length ?? 0} total</span>
          </div>

          <div className="stack-list compact-stack-list">
            {workspace?.categories.map((category) => (
              <div className="list-row" key={category.id}>
                <div>
                  <strong>{category.name}</strong>
                  <div className="muted-text">
                    {category.code} · {titleCaseWords(category.itemScope)}
                  </div>
                </div>
                <button
                  className="secondary-button"
                  type="button"
                  onClick={() => editCategory(category.id)}
                >
                  Edit
                </button>
              </div>
            ))}
          </div>

          <div className="card-header spacing-top">
            <h3>{categoryForm.id ? "Edit category" : "Add category"}</h3>
          </div>
          <div className="form-grid compact-form-grid">
            <label>
              <span>Name</span>
              <input
                value={categoryForm.name}
                onChange={(event) =>
                  setCategoryForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Code</span>
              <input
                value={categoryForm.code}
                onChange={(event) =>
                  setCategoryForm((current) => ({ ...current, code: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Parent category</span>
              <select
                value={categoryForm.parentId ?? ""}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    parentId: event.target.value || null
                  }))
                }
              >
                <option value="">No parent</option>
                {workspace?.categories
                  .filter((category) => category.id !== categoryForm.id)
                  .map((category) => (
                    <option key={category.id} value={category.id}>
                      {category.name}
                    </option>
                  ))}
              </select>
            </label>
            <label>
              <span>Scope</span>
              <select
                value={categoryForm.itemScope}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    itemScope: event.target.value
                  }))
                }
              >
                <option value="all">All</option>
                <option value="stock">Stock</option>
                <option value="menu">Menu</option>
                <option value="service">Service</option>
              </select>
            </label>
            <label>
              <span>Sort order</span>
              <input
                type="number"
                min="0"
                value={categoryForm.sortOrder}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    sortOrder: Number(event.target.value)
                  }))
                }
              />
            </label>
            <label className="form-span-2">
              <span>Notes</span>
              <textarea
                rows={3}
                value={categoryForm.notes ?? ""}
                onChange={(event) =>
                  setCategoryForm((current) => ({
                    ...current,
                    notes: event.target.value
                  }))
                }
              />
            </label>
          </div>
          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleCategorySave()}>
              {categoryForm.id ? "Save category" : "Add category"}
            </button>
            <button className="secondary-button" type="button" onClick={resetCategoryForm}>
              Clear form
            </button>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Units</h2>
            <span className="pill neutral">{workspace?.units.length ?? 0} available</span>
          </div>

          <div className="stack-list compact-stack-list">
            {workspace?.units.map((unit) => (
              <div className="list-row" key={unit.id}>
                <div>
                  <strong>{unit.name}</strong>
                  <div className="muted-text">
                    {unit.code} · {unit.allowFractional ? "Fractional" : "Whole only"}
                  </div>
                </div>
                {unit.isSystem ? (
                  <span className="pill success">System</span>
                ) : (
                  <button
                    className="secondary-button"
                    type="button"
                    onClick={() => editUnit(unit.id)}
                  >
                    Edit
                  </button>
                )}
              </div>
            ))}
          </div>

          <div className="card-header spacing-top">
            <h3>{unitForm.id ? "Edit custom unit" : "Add custom unit"}</h3>
          </div>
          <div className="form-grid compact-form-grid">
            <label>
              <span>Unit name</span>
              <input
                value={unitForm.name}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, name: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Code</span>
              <input
                value={unitForm.code}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, code: event.target.value }))
                }
              />
            </label>
            <label>
              <span>Symbol</span>
              <input
                value={unitForm.symbol}
                onChange={(event) =>
                  setUnitForm((current) => ({ ...current, symbol: event.target.value }))
                }
              />
            </label>
            <label className="toggle-row">
              <input
                type="checkbox"
                checked={unitForm.allowFractional}
                onChange={(event) =>
                  setUnitForm((current) => ({
                    ...current,
                    allowFractional: event.target.checked
                  }))
                }
              />
              <span>Allow fractional quantities</span>
            </label>
          </div>
          <div className="inline-actions spacing-top">
            <button className="primary-button" type="button" onClick={() => void handleUnitSave()}>
              {unitForm.id ? "Save unit" : "Add custom unit"}
            </button>
            <button className="secondary-button" type="button" onClick={resetUnitForm}>
              Clear form
            </button>
          </div>
        </article>
      </section>
    </div>
  );
}
