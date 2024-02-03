export function NaNTo(value: number, defaultValue: number): number {
  return isNaN(value) ? defaultValue : value
}

export function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max)
}
