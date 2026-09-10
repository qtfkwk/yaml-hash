/*!
Improved YAML Hash

If the YAML data you're working with is well-defined, and you want to write the necessary types, you
should use [`serde`] and [`serde_yaml_ng`].
Otherwise, [`yaml-hash`] provides a foundation for working with varied YAML data.

This crate provides the [`YamlHash`] struct, which is a wrapper for [`serde_yaml_ng::Mapping`], and
supports some additional capabilities.

[`serde`]: https://docs.rs/serde
[`yaml-hash`]: https://crates.io/crates/yaml-hash
[`serde_yaml_ng::Mapping`]: https://docs.rs/serde_yaml_ng/latest/serde_yaml_ng/struct.Mapping.html
*/

//--------------------------------------------------------------------------------------------------
// Crates

use {
    anyhow::{Result, anyhow},
    serde::{Deserialize, Serialize},
    serde_yaml_ng::Mapping,
    std::path::Path,
};

pub use serde_yaml_ng::Value;

//--------------------------------------------------------------------------------------------------
// Structs

/**
Improved YAML Hash

* Convert from YAML [`&str`] via `impl From<&str>`

* Convert to YAML [`String`] via `impl Display`

* Get a value for a dotted key as a [`YamlHash`] or [`serde_yaml_ng::Value`] via
  [`get`][`YamlHash::get`] and [`get_yaml`][`YamlHash::get_yaml`]

* Merge a [`YamlHash`] with another [`YamlHash`], YAML hash string, or YAML hash file to create a
  new [`YamlHash`] via [`merge`][`YamlHash::merge`], [`merge_str`][`YamlHash::merge_str`], or
  [`merge_file`][`YamlHash::merge_file`]

* Include a [`YamlHash`] as a type in a container type deriving/implementing [`serde::Serialize`] /
  [`serde::Deserialize`], such as a struct, newtype struct, enum variant, or enum struct variant
    * *Note that [`YamlHash`] flattens its internal [`serde_yaml_ng::Mapping`] so it can be
      serialized / deserialized to/from the same YAML string*
*/
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct YamlHash {
    #[serde(flatten)]
    data: Mapping,
}

impl YamlHash {
    /// Create a new empty [`YamlHash`]
    #[must_use]
    pub fn new() -> YamlHash {
        YamlHash::default()
    }

    /**
    Merge this [`YamlHash`] with another [`YamlHash`] to create a new [`YamlHash`]

    ```
    use yaml_hash::YamlHash;

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
    ");

    let other = YamlHash::from("\
    fruit:
      cherry:
        sweet: 1
        tart: 2
    ");

    assert_eq!(
        hash.merge(&other).to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2
    ");
    ```
    */
    #[must_use]
    pub fn merge(&self, other: &YamlHash) -> YamlHash {
        let mut r = self.clone();
        r.data = merge(&r.data, &other.data);
        r
    }

    /**
    Merge this [`YamlHash`] with a YAML hash [`&str`] to create a new [`YamlHash`]

    ```
    use yaml_hash::YamlHash;

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
    ");

    let hash = hash.merge_str("\
    fruit:
      cherry:
        sweet: 1
        tart: 2
    ").unwrap();

    assert_eq!(
        hash.to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2
    ");
    ```

    # Errors

    Returns an error if the YAML string is not a hash
    */
    pub fn merge_str(&self, s: &str) -> Result<YamlHash> {
        let mut r = self.clone();

        let s = if s.is_empty() { "{}" } else { s };

        let v: Value = serde_yaml_ng::from_str(s)?;

        if let Value::Mapping(m) = v {
            r.data = merge(&r.data, &m);

            Ok(r)
        } else {
            Err(anyhow!("YAML string is not a hash: {v:?}"))
        }
    }

    /**
    Merge this [`YamlHash`] with a YAML hash file to create a new [`YamlHash`]

    ```
    use yaml_hash::YamlHash;

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
    ");

    let hash = hash.merge_file("tests/b.yaml").unwrap();

    assert_eq!(
        hash.to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry: 3
    ");
    ```

    # Errors

    Returns an error if not able to read the file at the given path to a string
    */
    pub fn merge_file<P: AsRef<Path>>(&self, path: P) -> Result<YamlHash> {
        let yaml = std::fs::read_to_string(path)?;
        self.merge_str(&yaml)
    }

    /**
    Get the value for a dotted key as a [`Value`]

    ```
    use yaml_hash::{Value, YamlHash};

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2
    ");

    assert_eq!(
        hash.get_yaml("fruit.cherry.tart").unwrap(),
        Value::Number(2.into()),
    );
    ```

    # Errors

    Returns an error if the given key is not valid or the value is not a hash
    */
    pub fn get_yaml(&self, key: &str) -> Result<Value> {
        get_yaml(key, ".", &Value::Mapping(self.data.clone()), "")
    }

    /**
    Get a value for a dotted key as a [`YamlHash`]

    ```
    use yaml_hash::YamlHash;

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2
    ");

    assert_eq!(
        hash.get("fruit.cherry").unwrap(),
        YamlHash::from("\
    sweet: 1
    tart: 2
    "),
    );
    ```

    # Errors

    Returns an error if the given key is not valid or the value is not a hash
    */
    pub fn get(&self, key: &str) -> Result<YamlHash> {
        match self.get_yaml(key)?.as_mapping() {
            Some(data) => Ok(YamlHash { data: data.clone() }),
            None => Err(anyhow!("Value for {key:?} is not a hash")),
        }
    }
}

impl std::fmt::Display for YamlHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Ok(yaml) = serde_yaml_ng::to_string(&self.data) {
            write!(f, "{yaml}")
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl From<&str> for YamlHash {
    /// Create a [`YamlHash`] from a YAML hash string
    fn from(s: &str) -> YamlHash {
        YamlHash::default().merge_str(s).unwrap()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions

fn merge(a: &Mapping, b: &Mapping) -> Mapping {
    let mut r = a.clone();
    for (k, v) in b {
        if let Value::Mapping(bh) = v
            && let Some(Value::Mapping(rh)) = r.get(k)
        {
            if r.contains_key(k) {
                *r.get_mut(k).expect("get mut") = Value::Mapping(merge(rh, bh));
            } else {
                r.insert(k.clone(), Value::Mapping(merge(rh, bh)));
            }

            continue;
        }

        if r.contains_key(k) {
            *r.get_mut(k).expect("get mut") = v.clone();
        } else {
            r.insert(k.clone(), v.clone());
        }
    }

    r
}

fn get_yaml(key: &str, sep: &str, yaml: &Value, full: &str) -> Result<Value> {
    if key.is_empty() {
        return Ok(yaml.clone());
    }

    let mut s = key.split(sep);
    let this = s.next().unwrap();
    let next = s.collect::<Vec<&str>>().join(sep);

    match yaml {
        Value::Mapping(hash) => match hash.get(Value::String(this.to_string())) {
            Some(v) => {
                if next.is_empty() {
                    Ok(v.clone())
                } else {
                    let full = if full.is_empty() {
                        key.to_string()
                    } else {
                        format!("{full}.{this}")
                    };

                    get_yaml(&next, sep, v, &full)
                }
            }
            None => Err(anyhow!("Invalid key: {full:?}")),
        },
        _ => Err(anyhow!("Value for key {full:?} is not a hash")),
    }
}
