export class CineWeaveError extends Error {
  constructor(message, options = {}) {
    super(message);
    this.name = this.constructor.name;
    this.code = options.code ?? "CINEWEAVE_ERROR";
    this.details = options.details ?? null;
  }
}

export class ValidationError extends CineWeaveError {
  constructor(message, details = null) {
    super(message, { code: "VALIDATION_ERROR", details });
  }
}

export class CapabilityError extends CineWeaveError {
  constructor(message, details = null) {
    super(message, { code: "CAPABILITY_ERROR", details });
  }
}
