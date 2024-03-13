use yaml_hash::*;

#[test]
fn debug_pretty() {
    let hash = YamlHash::new();
    assert_eq!(format!("{:#?}", hash), "YamlHash {\n    data: {},\n}");
}

#[test]
fn debug() {
    let hash = YamlHash::new();
    assert_eq!(format!("{:?}", hash), "YamlHash { data: {} }");
}

#[test]
fn display() {
    let hash = YamlHash::new();
    assert_eq!(format!("{}", hash), "{}");
}

#[test]
fn to_string() {
    let hash = YamlHash::new();
    assert_eq!(hash.to_string(), "{}");
}

//--------------------------------------------------------------------------------------------------

#[test]
fn merge_str() {
    let hash = YamlHash::new();
    let yaml = "fruit:\n  apple: 1\n  banana: 2".to_string();
    let hash = hash.merge_str(&yaml).unwrap();
    assert_eq!(hash.to_string(), yaml);
}

#[test]
fn merge_multiple_str_str_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2";
    let yaml2 = "fruit:\n  cherry: 3";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2";
    let yaml2 = "fruit:\n  banana: 3";

    let result = "fruit:\n  apple: 1\n  banana: 3";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_2() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2";
    let yaml2 = "fruit:\n  apple: 3";

    let result = "fruit:\n  apple: 3\n  banana: 2";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_3() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3";
    let yaml2 = "fruit:\n  banana: 4";

    let result = "fruit:\n  apple: 1\n  banana: 4\n  cherry: 3";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_no_conflicts_deep() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1";
    let yaml2 = "fruit:\n  cherry:\n    tart: 2";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1\n    tart: 2";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_str_with_conflict_deep() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 1";
    let yaml2 = "fruit:\n  cherry:\n    sweet: 2";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 2";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

//--------------------------------------------------------------------------------------------------

#[test]
fn merge_file() {
    let hash = YamlHash::new();
    let hash = hash.merge_file("tests/a.yaml").unwrap();
    let result = "fruit:\n  apple: 1\n  banana: 2";
    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_str_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "fruit:\n  cherry: 3";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_str_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "fruit:\n  banana: 3";

    let result = "fruit:\n  apple: 1\n  banana: 3";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_str(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_file_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2";
    let yaml2 = "tests/b.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_str_file_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "fruit:\n  apple: 1\n  banana: 2";
    let yaml2 = "tests/c.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 3";

    let hash = hash.merge_str(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_file_no_conflicts() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "tests/b.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 2\n  cherry: 3";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

#[test]
fn merge_multiple_file_file_with_conflict() {
    let hash = YamlHash::new();

    let yaml1 = "tests/a.yaml";
    let yaml2 = "tests/c.yaml";

    let result = "fruit:\n  apple: 1\n  banana: 3";

    let hash = hash.merge_file(yaml1).unwrap();
    let hash = hash.merge_file(yaml2).unwrap();

    assert_eq!(hash.to_string(), result);
}

//--------------------------------------------------------------------------------------------------

#[test]
fn get() {
    let hash = YamlHash::new();

    let hash = hash
        .merge_str("fruit:\n  apple: 1\n  banana: 2\n  cherry:\n    sweet: 3")
        .unwrap();

    let fruit = hash.get("fruit").unwrap();
    assert_eq!(
        fruit.to_string(),
        "apple: 1\nbanana: 2\ncherry:\n  sweet: 3"
    );

    let apple = fruit.get_yaml("apple").unwrap();
    assert_eq!(apple, Yaml::Integer(1));

    let banana = fruit.get_yaml("banana").unwrap();
    assert_eq!(banana, Yaml::Integer(2));

    let cherry = fruit.get("cherry").unwrap();
    assert_eq!(cherry.to_string(), "sweet: 3");

    let sweet = cherry.get_yaml("sweet").unwrap();
    assert_eq!(sweet, Yaml::Integer(3));

    let sweet2 = hash.get_yaml("fruit.cherry.sweet").unwrap();
    assert_eq!(sweet2, Yaml::Integer(3));
}
