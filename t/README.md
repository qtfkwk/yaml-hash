Improved YAML Hash

If the YAML data you're working with is well-defined, and you want to write the necessary types, you
should use [`serde`] and [`serde_yaml_ng`].
Otherwise, [`yaml-hash`] provides a foundation for working with varied YAML data.

This crate provides the [`YamlHash`] struct, which is a wrapper for [`serde_yaml_ng::Mapping`], and
supports some additional capabilities.

[`serde`]: https://docs.rs/serde
[`serde_yaml_ng`]: https://docs.rs/serde_yaml_ng
[`serde_yaml_ng::Mapping`]: https://docs.rs/serde_yaml_ng/latest/serde_yaml_ng/struct.Mapping.html
[`yaml-hash`]: https://docs.rs/yaml-hash
[`YamlHash`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html

