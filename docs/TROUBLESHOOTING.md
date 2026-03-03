# Troubleshooting

## Common issues

## 1. "invalid blueprint" error

- Check `OsBlueprint::validate_report()` and read the `errors` field.
- Ensure `name` is not empty.
- Ensure `services.base` is `true`.

## 2. Unexpected warnings

Warnings are not hard failures by default.

- Review `ValidationReport::warnings`.
- For UEFI packaging, ensure `MemoryConfig::default_uefi()` is used.

## 3. CI failures

CI enforces:

- `cargo fmt -- --check`
- `cargo clippy ... -D warnings`
- `RUSTDOCFLAGS="-D warnings"`

Run the same commands locally to reproduce.
