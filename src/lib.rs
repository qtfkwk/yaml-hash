/*!
Improved YAML Hash

If the YAML data you're working with is well-defined and you want to write the necessary types, you
should use [`serde`] and [`serde_yaml`].

Otherwise, [`yaml_rust2`] provides a foundation for working with varied YAML data or when you don't
want to write the necessary types.

This crate provides the [`YamlHash`] struct, which is a wrapper for [`yaml_rust2::yaml::Hash`], and
supports some additional capabilities:

* Convert from [`&str`] via `impl From<&str>`
* Convert to [`String`] via `impl Display`
* Get a value for a dotted key as a [`YamlHash`] or [`yaml_rust2::Yaml`] via
  [`get`][`YamlHash::get`] and [`get_yaml`][`YamlHash::get_yaml`]; return the root hash if the key
  is `""`.
* Merge a [`YamlHash`] with another [`YamlHash`], YAML hash string, or YAML hash file to create a
  new [`YamlHash`] via [`merge`][`YamlHash::merge`], [`merge_str`][`YamlHash::merge_str`], or
  [`merge_file`][`YamlHash::merge_file`]

[`serde`]: https://docs.rs/serde
[`serde_yaml`]: https://docs.rs/serde_yaml
*/

//--------------------------------------------------------------------------------------------------

use {
    anyhow::{Result, anyhow},
    std::path::Path,
    yaml_rust2::{YamlEmitter, YamlLoader, yaml::Hash},
};

pub use yaml_rust2::Yaml;

//--------------------------------------------------------------------------------------------------

/**
Improved YAML Hash

* Convert from [`&str`] via `impl From<&str>`
* Convert to [`String`] via `impl Display`
* Get a value for a dotted key as a [`YamlHash`] or [`yaml_rust2::Yaml`] via
  [`get`][`YamlHash::get`] and [`get_yaml`][`YamlHash::get_yaml`]
* Merge a [`YamlHash`] with another [`YamlHash`], YAML hash string, or YAML hash file to create a
  new [`YamlHash`] via [`merge`][`YamlHash::merge`], [`merge_str`][`YamlHash::merge_str`], or
  [`merge_file`][`YamlHash::merge_file`]

*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct YamlHash {
    data: Hash,
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
      banana: 2\
    ");

    let other = YamlHash::from("\
    fruit:
      cherry:
        sweet: 1
        tart: 2\
    ");

    assert_eq!(
        hash.merge(&other).to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2\
        ",
    );
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
      banana: 2\
    ");

    let hash = hash.merge_str("\
    fruit:
      cherry:
        sweet: 1
        tart: 2\
    ").unwrap();

    assert_eq!(
        hash.to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2\
        ",
    );
    ```

    # Errors

    Returns an error if the YAML string is not a hash
    */
    pub fn merge_str(&self, s: &str) -> Result<YamlHash> {
        let mut r = self.clone();

        for doc in YamlLoader::load_from_str(s)? {
            if let Yaml::Hash(h) = doc {
                r.data = merge(&r.data, &h);
            } else {
                return Err(anyhow!("YAML string is not a hash: {doc:?}"));
            }
        }

        Ok(r)
    }

    /**
    Merge this [`YamlHash`] with a YAML hash file to create a new [`YamlHash`]

    ```
    use yaml_hash::YamlHash;

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2\
    ");

    let hash = hash.merge_file("tests/b.yaml").unwrap();

    assert_eq!(
        hash.to_string(),
        "\
    fruit:
      apple: 1
      banana: 2
      cherry: 3\
        ",
    );
    ```

    # Errors

    Returns an error if not able to read the file at the given path to a string
    */
    pub fn merge_file<P: AsRef<Path>>(&self, path: P) -> Result<YamlHash> {
        let yaml = std::fs::read_to_string(path)?;
        self.merge_str(&yaml)
    }

    /**
    Get the value for a dotted key as a [`Yaml`]

    ```
    use yaml_hash::{Yaml, YamlHash};

    let hash = YamlHash::from("\
    fruit:
      apple: 1
      banana: 2
      cherry:
        sweet: 1
        tart: 2\
    ");

    assert_eq!(
        hash.get_yaml("fruit.cherry.tart").unwrap(),
        Yaml::Integer(2),
    );
    ```

    # Errors

    Returns an error if the given key is not valid or the value is not a hash
    */
    pub fn get_yaml(&self, key: &str) -> Result<Yaml> {
        get_yaml(key, ".", &Yaml::Hash(self.data.clone()), "")
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
        tart: 2\
    ");

    assert_eq!(
        hash.get("fruit.cherry").unwrap(),
        YamlHash::from("\
    sweet: 1
    tart: 2\
        "),
    );
    ```

    # Errors

    Returns an error if the given key is not valid or the value is not a hash
    */
    pub fn get(&self, key: &str) -> Result<YamlHash> {
        match self.get_yaml(key)?.into_hash() {
            Some(data) => Ok(YamlHash { data }),
            None => Err(anyhow!("Value for {key:?} is not a hash")),
        }
    }
}

impl std::fmt::Display for YamlHash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut r = String::new();
        let mut emitter = YamlEmitter::new(&mut r);
        emitter.dump(&Yaml::Hash(self.data.clone())).unwrap();
        r.replace_range(..4, ""); // remove "---\n" at beginning
        write!(f, "{r}")
    }
}

impl From<&str> for YamlHash {
    /// Create a [`YamlHash`] from a YAML hash string
    fn from(s: &str) -> YamlHash {
        YamlHash::default().merge_str(s).unwrap()
    }
}

//--------------------------------------------------------------------------------------------------

fn merge(a: &Hash, b: &Hash) -> Hash {
    let mut r = a.clone();
    for (k, v) in b {
        if let Yaml::Hash(bh) = v
            && let Some(Yaml::Hash(rh)) = r.get(k)
        {
            if r.contains_key(k) {
                r.replace(k.clone(), Yaml::Hash(merge(rh, bh)));
            } else {
                r.insert(k.clone(), Yaml::Hash(merge(rh, bh)));
            }
            continue;
        }
        if r.contains_key(k) {
            r.replace(k.clone(), v.clone());
        } else {
            r.insert(k.clone(), v.clone());
        }
    }
    r
}

fn get_yaml(key: &str, sep: &str, yaml: &Yaml, full: &str) -> Result<Yaml> {
    if key.is_empty() {
        return Ok(yaml.clone());
    }

    let mut s = key.split(sep);
    let this = s.next().unwrap();
    let next = s.collect::<Vec<&str>>().join(sep);

    match yaml {
        Yaml::Hash(hash) => match hash.get(&Yaml::String(this.to_string())) {
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
