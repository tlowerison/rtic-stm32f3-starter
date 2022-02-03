[`probe-run`]: https://crates.io/crates/probe-run
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link
[`RTIC`]: https://rtic.rs

Project starter for the STM32F3 microcontroller.

> Quickly set up a [`probe-run`] + [`defmt`] + [`flip-link`] embedded project
> running on the [`RTIC`] scheduler on a STM32F3 microcontroller.

Based on https://github.com/rtic-rs/defmt-app-template

### Setup

```
cargo install flip-link
cargo install probe-run
cargo install cargo-generate
cargo generate \
    --git https://github.com/tlowerison/rtic-stm32f3-template \
    --branch main \
    --name my-app
```
