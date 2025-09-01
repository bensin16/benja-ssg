mod common;

use benja_ssg::SsgConfig;

#[test]
fn test_header_tags_from_file() {
    common::setup();

    let test_filename = r"tests/md_files/hello";
    let test_md = format!("{}.{}", test_filename, "md");
    println!("{test_md}");
    let args: Vec<String> = vec!["programname".to_string(), test_md];
    let cfg = SsgConfig::build(&args).unwrap();
    let _res = benja_ssg::run(cfg).unwrap();

    let expected = r#"
<!DOCTYPE html>
<html lang=en>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Replace Me!</title>
</head>
<body>
<h1>h1 test</h1>
<h2>h2 test</h2>
<h3>h3 test</h3>
<h4>h4 test</h4>
<h5>h5 test</h5>
<h6>h6 test</h6>
</body>
</html>
"#;

    let test_html = format!("{}.{}", test_filename, "html");
    let actual = common::read(&test_html).unwrap();

    let _res = std::fs::remove_file(test_html);
    assert_eq!(expected, actual);
}
