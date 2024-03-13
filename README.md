Improved YAML Hash

If the YAML data you're working with is well-defined and you want to write the necessary types, you
should use [`serde`] and [`serde_yaml`].

Otherwise, [`yaml_rust2`] provides a foundation for supporting either varied YAML data or in
instances where you don't want to write the necessary types.

This crate provides the [`YamlHash`] struct, which is a wrapper for [`yaml_rust2::yaml::Hash`], and
supports some additional capabilities:

* Convert from [`&str`] via `impl From<&str>`
* Convert to [`String`] via `impl Display`
* Get a value for a dotted key as a [`YamlHash`] or [`yaml_rust2::Yaml`] via
  [`get`][`YamlHash::get`], [`get_yaml`][`YamlHash::get_yaml`]
* Merge a [`YamlHash`] with another [`YamlHash`] or a YAML hash string to create a new [`YamlHash`]
  via [`merge`][`YamlHash::merge`], [`merge_str`][`YamlHash::merge_str`]

[`&str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
[`serde`]: https://docs.rs/serde
[`serde_yaml`]: https://docs.rs/serde_yaml
[`String`]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html
[`yaml_rust2`]: https://docs.rs/yaml-rust2
[`yaml_rust2::Yaml`]: https://docs.rs/yaml-rust2/latest/yaml_rust2/yaml/enum.Yaml.html
[`yaml_rust2::yaml::Hash`]: https://docs.rs/yaml-rust2/latest/yaml_rust2/yaml/type.Hash.html
[`YamlHash`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html
[`YamlHash::get`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html#method.get
[`YamlHash::get_yaml`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html#method.get_yaml
[`YamlHash::merge`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html#method.merge
[`YamlHash::merge_str`]: https://docs.rs/yaml-hash/latest/yaml_hash/struct.YamlHash.html#method.merge_str

