const HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang=en>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Replace Me!</title>
</head>
<body>
{body}
</body>
</html>
"#;

/// Parses markdown document and returns an HTML page
///
/// # Examples
///
/// ```
/// use benja_ssg::process_markdown;
///
/// let md = r"# Hello";
/// let html = process_markdown(md);
///
/// let expected = r#"
///<!DOCTYPE html>
///<html lang=en>
///<head>
///     <meta charset="utf-8">
///     <meta name="viewport" content="width=device-width, initial-scale=1.0">
///     <title>Replace Me!</title>
///</head>
///<body>
///<h1>Hello</h1>
///</body>
///</html>
///"#;
///
/// assert_eq!(html, expected);
/// ```
pub fn process_markdown(contents: &str) -> String {
    let mut html_body = String::new();

    for line in contents.lines() {
        let trimmed = line.trim_end();

        if trimmed.starts_with("#") {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            if level >= 1 && level <= 6 {
                let text = trimmed[level..].trim();
                html_body.push_str(&format!("<h{level}>{}</h{level}>\n", text));
            }
        }
    }

    HTML_TEMPLATE.replace("{body}", html_body.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_h1() {
        let contents = r"# Hello";
        let html = process_markdown(contents);
        let expected = r#"
<!DOCTYPE html>
<html lang=en>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Replace Me!</title>
</head>
<body>
<h1>Hello</h1>
</body>
</html>
"#;

        assert_eq!(html, expected);
    }

    #[test]
    fn process_h1_and_h2() {
        let contents = r"# Hello
## Hello 2";
        let html = process_markdown(contents);
        let expected = r#"
<!DOCTYPE html>
<html lang=en>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Replace Me!</title>
</head>
<body>
<h1>Hello</h1>
<h2>Hello 2</h2>
</body>
</html>
"#;

        assert_eq!(html, expected);
    }
}
