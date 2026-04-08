import { useCallback, useEffect, useMemo, useState } from "react";
import { useAppState } from "../../app/AppProvider";
import {
  loadInventoryWorkspace,
  recordInventoryMovement,
  saveInventoryStockRule
} from "../../shared/api";
import type {
  InventoryStockItem,
  InventoryWorkspace,
  SaveInventoryMovementInput,
  SaveInventoryStockRuleInput
} from "../../shared/types";
import {
  formatDateTime,
  formatQuantity,
  formatSignedQuantity,
  humanizeBoolean,
  humanizeMovementType
} from "../../shared/utils";

const emptyMovementForm: SaveInventoryMovementInput = {
  itemId: "",
  movementType: "stock_in",
  quantity: 0,
  unitCost: null,
  note: null
};

const emptyRuleForm: SaveInventoryStockRuleInput = {
  itemId: "",
  trackStock: true,
  reorderLevel: 0
};

function stockPill(item: InventoryStockItem) {
  if (!item.trackStock) {
    return <span className="pill neutral">Tracking off</span>;
  }

  if (item.lowStock) {
    return <span className="pill warning">Low stock</span>;
  }

  return <span className="pill success">Healthy</span>;
}

export function InventoryPage() {
  const { data, refresh } = useAppState();
  const [workspace, setWorkspace] = useState<InventoryWorkspace | null>(null);
  const [loading, setLoading] = useState(false);
  const [statusMessage, setStatusMessage] = useState("");
  const [search, setSearch] = useState("");
  const [lowStockOnly, setLowStockOnly] = useState(false);
  const [movementForm, setMovementForm] =
    useState<SaveInventoryMovementInput>(emptyMovementForm);
  const [ruleForm, setRuleForm] = useState<SaveInventoryStockRuleInput>(emptyRuleForm);

  const loadWorkspace = useCallback(async () => {
    setLoading(true);
    try {
      const next = await loadInventoryWorkspace();
      setWorkspace(next);

      const firstItem = next.stockItems[0] ?? null;
      setMovementForm((current) => {
        const currentExists = next.stockItems.some(
          (item) => item.itemId === current.itemId
        );
        if (currentExists) return current;
        return {
          ...current,
          itemId: firstItem?.itemId ?? ""
        };
      });

      setRuleForm((current) => {
        const currentItem = next.stockItems.find((item) => item.itemId === current.itemId);
        if (currentItem) {
          return {
            itemId: currentItem.itemId,
            trackStock: currentItem.trackStock,
            reorderLevel: currentItem.reorderLevel
          };
        }
        if (!firstItem) {
          return emptyRuleForm;
        }
        return {
          itemId: firstItem.itemId,
          trackStock: firstItem.trackStock,
          reorderLevel: firstItem.reorderLevel
        };
      });
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
    return workspace.stockItems.filter((item) => {
      const haystack = [
        item.itemName,
        item.sku,
        item.categoryName,
        item.unitCode
      ]
        .filter(Boolean)
        .join(" ")
        .toLowerCase();
      const matchesSearch = !query || haystack.includes(query);
      const matchesLowStock = !lowStockOnly || item.lowStock;
      return matchesSearch && matchesLowStock;
    });
  }, [lowStockOnly, search, workspace]);

  const selectedMovementItem = useMemo(() => {
    return workspace?.stockItems.find((item) => item.itemId === movementForm.itemId) ?? null;
  }, [movementForm.itemId, workspace]);

  const selectedRuleItem = useMemo(() => {
    return workspace?.stockItems.find((item) => item.itemId === ruleForm.itemId) ?? null;
  }, [ruleForm.itemId, workspace]);

  if (!data) return null;

  function useItemForRule(item: InventoryStockItem) {
    setRuleForm({
      itemId: item.itemId,
      trackStock: item.trackStock,
      reorderLevel: item.reorderLevel
    });
    setStatusMessage(`Loaded stock rule for ${item.itemName}.`);
  }

  async function handleMovementSave() {
    if (!movementForm.itemId) {
      setStatusMessage("Select a stock item first.");
      return;
    }
    if (movementForm.quantity <= 0) {
      setStatusMessage("Movement quantity must be greater than zero.");
      return;
    }

    setStatusMessage("Recording stock movement…");
    try {
      await recordInventoryMovement({
        ...movementForm,
        quantity: Number(movementForm.quantity) || 0,
        unitCost:
          movementForm.unitCost !== null && movementForm.unitCost >= 0
            ? Number(movementForm.unitCost)
            : null,
        note: movementForm.note?.trim() ? movementForm.note.trim() : null
      });
      await loadWorkspace();
      await refresh();
      setMovementForm((current) => ({
        ...current,
        quantity: 0,
        unitCost: null,
        note: null
      }));
      setStatusMessage("Inventory movement recorded locally.");
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to record movement."
      );
    }
  }

  async function handleRuleSave() {
    if (!ruleForm.itemId) {
      setStatusMessage("Select a stock item for stock rules.");
      return;
    }

    setStatusMessage("Saving stock rule…");
    try {
      const updated = await saveInventoryStockRule({
        ...ruleForm,
        reorderLevel: Number(ruleForm.reorderLevel) || 0
      });
      await loadWorkspace();
      await refresh();
      setRuleForm({
        itemId: updated.itemId,
        trackStock: updated.trackStock,
        reorderLevel: updated.reorderLevel
      });
      setStatusMessage(`Stock rule saved for ${updated.itemName}.`);
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to save stock rule."
      );
    }
  }

  return (
    <div className="page-grid">
      <section className="hero-card">
        <div>
          <div className="section-kicker">Patch 4 inventory ledger core</div>
          <h2>{data.activeBusiness.name} stock workspace</h2>
          <p>
            Track on-hand quantities locally through stock movements. Patch 4 adds
            quantity history, reorder monitoring, and inventory-aware export support
            without needing any cloud service.
          </p>
        </div>
        <div className="hero-actions align-start">
          <span className="meta-chip">
            Tracked items: {workspace?.summary.totalTrackedItems ?? 0}
          </span>
          <span className="meta-chip">
            Low stock: {workspace?.summary.lowStockItems ?? 0}
          </span>
          <span className="meta-chip">
            Movements: {workspace?.summary.movementCount ?? 0}
          </span>
        </div>
      </section>

      <section className="card-grid card-grid-4">
        <article className="card">
          <div className="card-label">Tracked stock items</div>
          <div className="kpi-value">{workspace?.summary.totalTrackedItems ?? 0}</div>
          <p className="card-note">
            Items marked for stock tracking now carry a movement history locally.
          </p>
        </article>
        <article className="card">
          <div className="card-label">Low stock alerts</div>
          <div className="kpi-value">{workspace?.summary.lowStockItems ?? 0}</div>
          <p className="card-note">
            Based on reorder thresholds set per stock item.
          </p>
        </article>
        <article className="card">
          <div className="card-label">Inventory movements</div>
          <div className="kpi-value">{workspace?.summary.movementCount ?? 0}</div>
          <p className="card-note">
            Includes opening balances, stock changes, and catalog sync adjustments.
          </p>
        </article>
        <article className="card">
          <div className="card-label">Quantity on hand</div>
          <div className="kpi-value">
            {formatQuantity(workspace?.summary.totalQuantityOnHand ?? 0)}
          </div>
          <p className="card-note">
            Total across currently tracked stock items in the active business.
          </p>
        </article>
      </section>

      <div className="status-banner">{statusMessage || "Inventory workspace ready."}</div>

      <section className="split-grid inventory-main-grid">
        <article className="card">
          <div className="card-header">
            <h2>Stock items</h2>
            <span className="pill neutral">
              {loading ? "Refreshing…" : `${filteredItems.length} shown`}
            </span>
          </div>

          <div className="inventory-toolbar">
            <input
              placeholder="Search item, SKU, category, unit"
              value={search}
              onChange={(event) => setSearch(event.target.value)}
            />
            <label className="checkbox-field compact-checkbox">
              <input
                type="checkbox"
                checked={lowStockOnly}
                onChange={(event) => setLowStockOnly(event.target.checked)}
              />
              <span>Low stock only</span>
            </label>
          </div>

          <div className="stack-list inventory-item-list">
            {filteredItems.length > 0 ? (
              filteredItems.map((item) => (
                <div className="list-row inventory-item-row" key={item.itemId}>
                  <div>
                    <strong>{item.itemName}</strong>
                    <div className="muted-text">
                      {item.categoryName ?? "Uncategorized"}
                      {item.sku ? ` · ${item.sku}` : ""}
                      {item.unitCode ? ` · ${item.unitCode}` : ""}
                    </div>
                    <div className="inventory-inline-meta">
                      {stockPill(item)}
                      <span className="pill neutral">
                        Tracking {humanizeBoolean(item.trackStock).toLowerCase()}
                      </span>
                    </div>
                  </div>
                  <div className="inventory-qty-block">
                    <strong>{formatQuantity(item.stockQuantity)}</strong>
                    <span className="muted-text">
                      Reorder at {formatQuantity(item.reorderLevel)}
                    </span>
                    <button
                      className="secondary-button small-button"
                      type="button"
                      onClick={() => useItemForRule(item)}
                    >
                      Edit rule
                    </button>
                  </div>
                </div>
              ))
            ) : (
              <p className="muted-text">
                No stock items match the current filter.
              </p>
            )}
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Recent stock movements</h2>
            <span className="pill neutral">
              {workspace?.recentMovements.length ?? 0} loaded
            </span>
          </div>
          <div className="stack-list inventory-movement-list">
            {workspace?.recentMovements.length ? (
              workspace.recentMovements.map((movement) => (
                <div className="list-row inventory-movement-row" key={movement.id}>
                  <div>
                    <strong>{movement.itemName}</strong>
                    <div className="muted-text">
                      {humanizeMovementType(movement.movementType)}
                      {movement.unitCode ? ` · ${movement.unitCode}` : ""}
                      {movement.sku ? ` · ${movement.sku}` : ""}
                    </div>
                    {movement.note ? (
                      <div className="muted-text small-text">{movement.note}</div>
                    ) : null}
                  </div>
                  <div className="inventory-qty-block">
                    <strong
                      className={
                        movement.quantityDelta < 0 ? "qty-negative" : "qty-positive"
                      }
                    >
                      {formatSignedQuantity(movement.quantityDelta)}
                    </strong>
                    <span className="muted-text">
                      After: {formatQuantity(movement.quantityAfter)}
                    </span>
                    <span className="muted-text small-text">
                      {formatDateTime(movement.occurredAt)}
                    </span>
                  </div>
                </div>
              ))
            ) : (
              <p className="muted-text">No movements recorded yet.</p>
            )}
          </div>
        </article>
      </section>

      <section className="split-grid inventory-form-grid">
        <article className="card">
          <div className="card-header">
            <h2>Record movement</h2>
            <span className="pill success">Local ledger</span>
          </div>
          <p className="card-note">
            Use stock in, stock out, and adjustment moves to keep on-hand balances
            accurate without overwriting history.
          </p>
          <div className="form-grid">
            <label>
              <span>Stock item</span>
              <select
                value={movementForm.itemId}
                onChange={(event) =>
                  setMovementForm((current) => ({
                    ...current,
                    itemId: event.target.value
                  }))
                }
              >
                <option value="">Select stock item</option>
                {workspace?.stockItems.map((item) => (
                  <option key={item.itemId} value={item.itemId}>
                    {item.itemName}
                  </option>
                ))}
              </select>
            </label>

            <label>
              <span>Movement type</span>
              <select
                value={movementForm.movementType}
                onChange={(event) =>
                  setMovementForm((current) => ({
                    ...current,
                    movementType: event.target.value
                  }))
                }
              >
                <option value="stock_in">Stock in</option>
                <option value="stock_out">Stock out</option>
                <option value="adjustment_in">Adjustment in</option>
                <option value="adjustment_out">Adjustment out</option>
              </select>
            </label>

            <label>
              <span>Quantity</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={movementForm.quantity}
                onChange={(event) =>
                  setMovementForm((current) => ({
                    ...current,
                    quantity: Number(event.target.value)
                  }))
                }
              />
            </label>

            <label>
              <span>Unit cost (optional)</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={movementForm.unitCost ?? ""}
                onChange={(event) =>
                  setMovementForm((current) => ({
                    ...current,
                    unitCost: event.target.value === "" ? null : Number(event.target.value)
                  }))
                }
              />
            </label>

            <label className="form-span-2">
              <span>Note</span>
              <textarea
                rows={3}
                placeholder="Reason, supplier batch, stock check note"
                value={movementForm.note ?? ""}
                onChange={(event) =>
                  setMovementForm((current) => ({
                    ...current,
                    note: event.target.value
                  }))
                }
              />
            </label>
          </div>

          <div className="detail-list compact-detail-list">
            <div>
              <span>Selected item</span>
              <code>{selectedMovementItem?.itemName ?? "—"}</code>
            </div>
            <div>
              <span>Current quantity</span>
              <code>{formatQuantity(selectedMovementItem?.stockQuantity ?? 0)}</code>
            </div>
            <div>
              <span>Tracking</span>
              <code>{humanizeBoolean(selectedMovementItem?.trackStock ?? false)}</code>
            </div>
            <div>
              <span>Reorder level</span>
              <code>{formatQuantity(selectedMovementItem?.reorderLevel ?? 0)}</code>
            </div>
          </div>

          <div className="inline-actions">
            <button
              className="primary-button"
              type="button"
              onClick={() => void handleMovementSave()}
              disabled={!selectedMovementItem?.trackStock}
            >
              Record Movement
            </button>
          </div>
        </article>

        <article className="card">
          <div className="card-header">
            <h2>Stock rule</h2>
            <span className="pill neutral">Per item</span>
          </div>
          <p className="card-note">
            Adjust reorder level and stock tracking without removing historical ledger
            rows already recorded for the item.
          </p>
          <div className="form-grid">
            <label className="form-span-2">
              <span>Stock item</span>
              <select
                value={ruleForm.itemId}
                onChange={(event) => {
                  const selected = workspace?.stockItems.find(
                    (item) => item.itemId === event.target.value
                  );
                  setRuleForm({
                    itemId: event.target.value,
                    trackStock: selected?.trackStock ?? true,
                    reorderLevel: selected?.reorderLevel ?? 0
                  });
                }}
              >
                <option value="">Select stock item</option>
                {workspace?.stockItems.map((item) => (
                  <option key={item.itemId} value={item.itemId}>
                    {item.itemName}
                  </option>
                ))}
              </select>
            </label>

            <label className="checkbox-field">
              <input
                type="checkbox"
                checked={ruleForm.trackStock}
                onChange={(event) =>
                  setRuleForm((current) => ({
                    ...current,
                    trackStock: event.target.checked
                  }))
                }
              />
              <span>Track stock for this item</span>
            </label>

            <label>
              <span>Reorder level</span>
              <input
                type="number"
                min="0"
                step="0.01"
                value={ruleForm.reorderLevel}
                onChange={(event) =>
                  setRuleForm((current) => ({
                    ...current,
                    reorderLevel: Number(event.target.value)
                  }))
                }
              />
            </label>
          </div>

          <div className="detail-list compact-detail-list">
            <div>
              <span>Selected item</span>
              <code>{selectedRuleItem?.itemName ?? "—"}</code>
            </div>
            <div>
              <span>Current quantity</span>
              <code>{formatQuantity(selectedRuleItem?.stockQuantity ?? 0)}</code>
            </div>
            <div>
              <span>Low stock</span>
              <code>{humanizeBoolean(selectedRuleItem?.lowStock ?? false)}</code>
            </div>
            <div>
              <span>Last updated</span>
              <code>{formatDateTime(selectedRuleItem?.updatedAt)}</code>
            </div>
          </div>

          <div className="inline-actions">
            <button
              className="secondary-button"
              type="button"
              onClick={() => void handleRuleSave()}
            >
              Save Stock Rule
            </button>
          </div>
        </article>
      </section>
    </div>
  );
}
