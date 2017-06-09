

## Procedural Macros & Custom Derive Challenge

**Objectives**
- Implement the `CustomDebug` derive to output the fields of a struct in a custom way.
    - The `CustomDebug` derive will implement the `Debug` trait for the struct.
    - Use `syn::DeriveInput` to get the fields of the struct.
    - Iterate over each field and output the field in a nice way.
- Extra: Use custom attributes to affect your `CustomDebug` output.
    - Only output fields that are tagged with the `#[debug]` attribute.

**Helpful Documentation**
- [Procedural Macros & Custom Derive (Rust Book)](https://doc.rust-lang.org/book/first-edition/procedural-macros.html)
- [`quote` Crate Documentation](https://docs.rs/quote/0.3.15/quote/)
- [`syn` Crate Documentation](https://dtolnay.github.io/syn/syn/index.html) (specifically `DeriveInput`, `Field`)
