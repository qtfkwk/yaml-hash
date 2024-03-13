#![doc = include_str!("../README.md")]

//--------------------------------------------------------------------------------------------------

use anyhow::{anyhow, Result};
use yaml_rust2::{yaml::Hash, YamlEmitter, YamlLoader};

pub use yaml_rust2::Yaml;

//--------------------------------------------------------------------------------------------------

/**
Improved YAML Hash

* Convert from [`&str`] (via `impl From<&str>`
* Convert to [`String`] (via `impl Display`)
* Get a value for a dotted key as a [`YamlHash`] or [`yaml_rust2::Yaml`] via
  [`get`][`YamlHash::get`], [`get_yaml`][`YamlHash::get_yaml`]
* Merge a [`YamlHash`] with another [`YamlHash`] or a YAML hash string to create a new [`YamlHash`]
  via [`merge`][`YamlHash::merge`], [`merge_str`][`YamlHash::merge_str`]

*/
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct YamlHash {
    data: Hash,
}

impl YamlHash {
    /// Create a new empty [`YamlHash`]
    pub fn new() -> YamlHash {
        YamlHash::default()
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
        hash.merge(&other).unwrap().to_string(),
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
    pub fn merge(&self, other: &YamlHash) -> Result<YamlHash> {
        let mut r = self.clone();
        r.data = merge(&r.data, &other.data);
        Ok(r)
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
    */
    pub fn get_yaml(&self, key: &str) -> Result<Yaml> {
        get_yaml(key, &Yaml::Hash(self.data.clone()), "")
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
    for (k, v) in b.iter() {
        if let Yaml::Hash(bh) = v {
            if let linked_hash_map::Entry::Occupied(e) = r.entry(k.clone()) {
                if let Yaml::Hash(rh) = e.get().clone() {
                    r.entry(k.clone())
                        .and_modify(|e| *e = Yaml::Hash(merge(&rh, bh)))
                        .or_insert_with(|| Yaml::Hash(merge(&rh, bh)));
                    continue;
                }
            }
        }
        r.entry(k.clone())
            .and_modify(|e| *e = v.clone())
            .or_insert_with(|| v.clone());
    }
    r
}

fn get_yaml(key: &str, yaml: &Yaml, full_key: &str) -> Result<Yaml> {
    let mut s = key.split('.');
    let key = s.next().unwrap();
    let yaml_key = Yaml::String(key.to_string());
    let next_key = s.collect::<Vec<&str>>().join(".");

    match yaml {
        Yaml::Hash(hash) => match hash.get(&yaml_key) {
            Some(v) => {
                if next_key.is_empty() {
                    Ok(v.clone())
                } else {
                    let full_key = if full_key.is_empty() {
                        key.to_string()
                    } else {
                        format!("{full_key}.{key}")
                    };
                    get_yaml(&next_key, v, &full_key)
                }
            }
            None => Err(anyhow!("Invalid key: {full_key:?}")),
        },
        _ => Err(anyhow!("Value for key {full_key:?} is not a hash")),
    }
}
