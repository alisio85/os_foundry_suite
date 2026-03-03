# Observability guide

## Purpose

`os_foundry_suite` treats observability as a first-class concern by encouraging consistent conventions across kernel, services, and test harnesses.

## Integration

When the `observatory` feature is enabled:

- The underlying `os_observatory` crate is re-exported as `os_foundry_suite::crates::observatory`.

Downstream projects are encouraged to:

- define a small set of boot milestones
- emit structured events that tests can assert on

## Recommendations

- Prefer a stable serial log format for early boot.
- Ensure panic reporting includes build identifiers.
