import { ValidationError } from "./errors.js";

export function assert(condition, message, details = null) {
  if (!condition) {
    throw new ValidationError(message, details);
  }
}

export function assertNonEmptyString(value, fieldName) {
  assert(typeof value === "string" && value.trim().length > 0, `${fieldName} must be a non-empty string`, {
    fieldName,
    value,
  });
}

export function assertArray(value, fieldName) {
  assert(Array.isArray(value), `${fieldName} must be an array`, { fieldName, value });
}
