# Changelog

* 0.1.0 (2024-03-12): Initial release
    * 0.1.1 (2024-03-12): Fix readme/doc
    * 0.1.2 (2024-03-13): Fix readme/doc
* 0.2.0 (2024-03-13): Add `merge_file` method and integration tests; fix readme/doc
* 0.3.0 (2024-03-14): Return the root as a `YamlHash` or `yaml-rust2::yaml::Hash` for `get*`; add `Makefile`
* 0.4.0 (2024-07-24): Update dependencies; upstream yaml-rust2 replaced linked-hash-map with hashlink, whose entry/and_modify/or_insert_with pattern moves entries to the end, but can use the contains_key/replace/insert pattern instead to maintain insertion order; this also alleviates the need to use the `Entry::Occupied` enum variant directly, which allows removing the secondary upstream dependency; replace make/`Makefile` with [`mkrs`]/[`Makefile.md`]
    * 0.4.1 (2024-08-23): Fix changelog; fix makefile; update dependencies
    * 0.4.2 (2024-10-24): Update dependencies
    * 0.4.3 (2024-12-04): Update dependencies
    * 0.4.4 (2025-02-20): Update dependencies
    * 0.4.5 (2025-04-16): Update dependencies
* 0.5.0 (2025-08-28): Update dependencies; 2024 edition
    * 0.5.1 (2025-10-27): Update dependencies
* 0.6.0 (2025-11-13): clippy fixes
    * 0.6.1 (2025-11-14): Update dependencies (none); fix changelog

[`mkrs`]: https://crates.io/crates/mkrs

