export function formatDateTime(value: string | null | undefined): string {
  if (!value) return "—";
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) return value;
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short"
  }).format(parsed);
}

export function humanizeBoolean(value: boolean): string {
  return value ? "Enabled" : "Disabled";
}

export function classNames(...values: Array<string | false | null | undefined>): string {
  return values.filter(Boolean).join(" ");
}

export function titleCaseWords(value: string): string {
  return value
    .split(/[-_\s]+/)
    .filter(Boolean)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(" ");
}

export function formatSequencePreview(
  prefix: string,
  nextNumber: number,
  padding: number
): string {
  const normalizedPadding = Math.max(1, Number.isFinite(padding) ? padding : 1);
  const normalizedNumber = Math.max(1, Number.isFinite(nextNumber) ? nextNumber : 1);
  return `${prefix}${String(normalizedNumber).padStart(normalizedPadding, "0")}`;
}

export function formatModuleList(modules: string[]): string {
  if (modules.length === 0) return "No modules enabled";
  return modules.map(titleCaseWords).join(", ");
}

export function formatCurrency(value: number, currencyCode = "INR"): string {
  return new Intl.NumberFormat(undefined, {
    style: "currency",
    currency: currencyCode,
    maximumFractionDigits: 2
  }).format(Number.isFinite(value) ? value : 0);
}

export function linesFromMultilineValue(value: string): string[] {
  return value
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

export function multilineValueFromLines(values: string[]): string {
  return values.join("\n");
}
