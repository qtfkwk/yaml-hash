//--------------------------------------------------------------------------------------------------
// Crates

use {
    serde::{Deserialize, Serialize},
    yaml_hash::*,
};

//--------------------------------------------------------------------------------------------------
// Tests

#[test]
fn debug_pretty() {
    let hash = YamlHash::new();

    assert_eq!(
        format!("{:#?}", hash),
        "YamlHash {\n    data: Mapping {},\n}",
    );
}

#[test]
fn debug() {
    let hash = YamlHash::new();

    assert_eq!(format!("{:?}", hash), "YamlHash { data: Mapping {} }");
}

#[test]
fn display() {
    let hash = YamlHash::new();

    assert_eq!(format!("{}", hash), "{}\n");
}

#[test]
fn to_string() {
    let hash = YamlHash::new();

    assert_eq!(hash.to_string(), "{}\n");
}

#[test]
fn empty() {
    let hash = YamlHash::from("");

    assert_eq!(format!("{:?}", hash), "YamlHash { data: Mapping {} }");
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[test]
fn merge_str() {
    let hash = YamlHash::new();
    let yaml = "fruit:\n  apple: 1\n  banana: 2\n";
    let hash = hash.merge_str(&yaml).unwrap();

    assert_eq!(hash.to_string(), yaml);
}

#[test]
fn merge_multiple_str_str_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n";
    let yaml2 = "fruit:\n  cherry: 3\n";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n";
    let yaml2 = "fruit:\n  banana: 3\n";

    let result = "fruit:\n  apple: 1\n  banana: 3\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_2() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n";
    let yaml2 = "fruit:\n  apple: 3\n";

    let result = "fruit:\n  apple: 3\n  banana: 2\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_3() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3\n";
    let yaml2 = "fruit:\n  banana: 4\n";

    let result = "fruit:\n  apple: 1\n  banana: 4\n  cherry: 3\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_no_conflicts_deep() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1\n";
    let yaml2 = "fruit:\n  cherry:\n    tart: 2\n";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1\n    tart: 2\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_deep() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1\n";
    let yaml2 = "fruit:\n  cherry:\n    sweet: 2\n";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 2\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[test]
fn merge_file() {
    let hash = YamlHash::new();
    let hash = hash.merge_file("tests/a.yaml").unwrap();
    let result = "fruit:\n  apple: 1\n  banana: 2\n";

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_str_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "fruit:\n  cherry: 3\n";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3\n";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_str_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "fruit:\n  banana: 3\n";

    let result = "fruit:\n  apple: 1\n  banana: 3\n";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_file_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n";
    let yaml2 = "tests/b.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_file_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n";
    let yaml2 = "tests/c.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 3\n";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_file_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "tests/b.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3\n";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_file_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "tests/c.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 3\n";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[test]
fn get() {
    let hash = YamlHash::new();

    let yaml = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 3\n";

    let hash = hash.merge_str(yaml).unwrap();

    let root = hash.get("").unwrap();
    assert_eq!(root.to_string(), yaml);

    let fruit = hash.get("fruit").unwrap();
    assert_eq!(
        fruit.to_string(),
        "apple: 1\nbanana: 2\ncherry:\n  sweet: 3\n",
    );

    let apple = fruit.get_yaml("apple").unwrap();
    assert_eq!(apple, Value::Number(1.into()));

    let banana = fruit.get_yaml("banana").unwrap();
    assert_eq!(banana, Value::Number(2.into()));

    let cherry = fruit.get("cherry").unwrap();
    assert_eq!(cherry.to_string(), "sweet: 3\n");

    let sweet = cherry.get_yaml("sweet").unwrap();
    assert_eq!(sweet, Value::Number(3.into()));

    let sweet2 = hash.get_yaml("fruit.cherry.sweet").unwrap();
    assert_eq!(sweet2, Value::Number(3.into()));
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

#[test]
fn serde_struct_newtype() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Something(YamlHash);

    let yaml = "\
fruit:
  apple: 1
  banana: 2
\
    ";

    let something = Something(YamlHash::from(yaml));

    let serialized = serde_yaml_ng::to_string(&something).expect("serialize something");
    assert_eq!(serialized, yaml);

    let deserialized: Something =
        serde_yaml_ng::from_str(&serialized).expect("deserialize something");
    assert_eq!(deserialized, something);
}

#[test]
fn serde_struct() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Something {
        #[serde(flatten)]
        yaml: YamlHash,
    }

    let yaml = "\
fruit:
  apple: 1
  banana: 2
\
    ";

    let something = Something {
        yaml: YamlHash::from(yaml),
    };

    let serialized = serde_yaml_ng::to_string(&something).expect("serialize something");
    assert_eq!(serialized, yaml);

    let deserialized: Something =
        serde_yaml_ng::from_str(&serialized).expect("deserialize something");
    assert_eq!(deserialized, something);
}

#[test]
fn serde_enum() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    enum Something {
        Yaml(YamlHash),
    }

    let yaml = "\
fruit:
  apple: 1
  banana: 2
\
    ";

    let something_yaml = "\
!Yaml
fruit:
  apple: 1
  banana: 2
\
    ";

    let something = Something::Yaml(YamlHash::from(yaml));

    let serialized = serde_yaml_ng::to_string(&something).expect("serialize something");
    assert_eq!(serialized, something_yaml);

    let deserialized: Something =
        serde_yaml_ng::from_str(&serialized).expect("deserialize something");
    assert_eq!(deserialized, something);
}

#[test]
fn serde_enum_struct_variant() {
    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    enum Something {
        Yaml {
            #[serde(flatten)]
            yaml: YamlHash,
        },
    }

    let yaml = "\
fruit:
  apple: 1
  banana: 2
\
    ";

    let something_yaml = "\
!Yaml
fruit:
  apple: 1
  banana: 2
\
    ";

    let something = Something::Yaml {
        yaml: YamlHash::from(yaml),
    };

    let serialized = serde_yaml_ng::to_string(&something).expect("serialize something");
    assert_eq!(serialized, something_yaml);

    let deserialized: Something =
        serde_yaml_ng::from_str(&serialized).expect("deserialize something");
    assert_eq!(deserialized, something);
}
