# Project guidance

## OCPP reference

- Before changing OCPP-related code, consult the matching version’s reference on [ocpp.md](https://ocpp.md/): OCPP 1.6J, 2.0.1, or 2.1. Use it to understand protocol fields, constraints, behavior, and any areas it marks as requiring a deployment-specific decision; do not infer protocol details from names or existing code alone.
- `ocpp.md` is a practical, non-normative reference. Verify normative requirements against the official OCPP specification and schemas, especially when adding or changing serialized fields or validation. Follow project schema-validation patterns where applicable.
- If the reference and official sources do not resolve an ambiguity, ask for clarification rather than inventing protocol behavior.

## Project structure

- This repository is a Rust library (`rust-ocpp`, edition 2021) implementing OCPP data types and messages.
- Implementations live under `src/v1_6`, `src/v2_0_1`, and `src/v2_1`. The `v2_1` implementation is work in progress.
- Each protocol version is enabled through a Cargo feature: `v1_6`, `v2_0_1`, or `wip_v2_1`. There is no default feature.
- Shared test infrastructure and schema validation live under `src/tests`.

## Implementation conventions

- Follow the existing version-specific module layout and naming conventions for messages and types.
- Preserve the wire format: message structs generally use Serde with `rename_all = "camelCase"`; optional fields should follow neighboring types’ serialization behavior.
- Keep protocol versions isolated in their respective modules unless a change is intentionally shared.
- Match validation constraints and required/optional fields to the matching `ocpp.md` reference and the relevant official schema/specification.

## Build and quality checks

- Use the stable Rust toolchain specified by `rust-toolchain.toml`.
- Format with `cargo fmt`.
- CI runs `cargo fmt -- --check`, `cargo clippy --tests --features="v1_6,v2_0_1" -- -D warnings`, `cargo build --features="v1_6,v2_0_1"`, and `cargo test --features="v1_6,v2_0_1"`.
- For changes involving the work-in-progress OCPP 2.1 module, also check the relevant feature with `wip_v2_1` enabled.
