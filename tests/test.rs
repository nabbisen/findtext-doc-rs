use findtext_doc::search;

const RESOURCE_FILEPATH_01: &str = "./tests/fixtures/file1.docx";
const DUMMY_FILEPATH_01: &str = "./tests/fixtures/_file0.docx";

#[test]
fn found_01() {
    const EXPECT: bool = true;

    let ret = search("hej", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);

    let ret = ret.unwrap();
    assert_eq!(ret, EXPECT, "Expected Ok({}), got {:?}", EXPECT, ret);
}

#[test]
fn missing_01() {
    const EXPECT: bool = false;

    let ret = search("heJ", RESOURCE_FILEPATH_01);

    assert!(ret.is_ok(), "Expected Ok, got {:?}", ret);

    let ret = ret.unwrap();
    assert_eq!(ret, EXPECT, "Expected Ok({}), got {:?}", EXPECT, ret);
}

#[test]
fn error_01() {
    let ret = search("hej", DUMMY_FILEPATH_01);

    assert!(ret.is_err(), "Expected Err, got {:?}", ret);
}
