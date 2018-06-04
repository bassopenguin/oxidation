extern crate regex;

use std::borrow::Cow;
use regex::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_h1s() {
        let formatter: Formatter = H1Formatter::new();
        let input: &str = "
# A simple page

## With minimal content

Including a list below

- Item #1
- Item #2

# And two h1 elements";
        let expected: &str = "
<h1>A simple page</h1>

## With minimal content

Including a list below

- Item #1
- Item #2

<h1>And two h1 elements</h1>";
        assert_eq!(expected, format(input, formatter));
    }

    #[test]
    fn it_converts_h2s() {
        let formatter: Formatter = H2Formatter::new();
        let input: &str = "
# Title

## Subtitle

Paragraph one

## Paragraph # 2

- List item that is _important_";
        let expected: &str = "
# Title

<h2>Subtitle</h2>

Paragraph one

<h2>Paragraph # 2</h2>

- List item that is _important_";
        assert_eq!(expected, format(input, formatter));
    }

    #[test]
    fn it_converts_italics() {
        let formatter: Formatter = ItalicsFormatter::new();
        let input: &str = "
# Title

## Subtitle

Paragraph with _something italicized_ inside it. And with a snake_case variable

# And a title with an _italics_ word";
        let expected: &str = "
# Title

## Subtitle

Paragraph with <i>something italicized</i> inside it. And with a snake_case variable

# And a title with an <i>italics</i> word";
        assert_eq!(expected, format(input, formatter));
    }

    #[test]
    fn it_converts_multiple_styles() {
        let mut formatter: Formatter = H1Formatter::new();
        let input: &str = "
# Title

## Subtitle

Paragraph with _something italicized_ inside it. And with a snake_case variable

# And a title with an _italics_ word";
        let expected: &str = "
<h1>Title</h1>

## Subtitle

Paragraph with _something italicized_ inside it. And with a snake_case variable

<h1>And a title with an _italics_ word</h1>";
        let h1_replaced: Cow<str> = format(input, formatter);
        assert_eq!(expected, h1_replaced);
        let expected_h2: &str = "
<h1>Title</h1>

<h2>Subtitle</h2>

Paragraph with _something italicized_ inside it. And with a snake_case variable

<h1>And a title with an _italics_ word</h1>";
        formatter = H2Formatter::new();
        let h2_replaced = format(&h1_replaced, formatter);
        assert_eq!(expected_h2, h2_replaced);
        let expected_italics: &str = "
<h1>Title</h1>

<h2>Subtitle</h2>

Paragraph with <i>something italicized</i> inside it. And with a snake_case variable

<h1>And a title with an <i>italics</i> word</h1>";
        formatter = ItalicsFormatter::new();
        let italics_replaced = format(&h2_replaced, formatter);
        assert_eq!(expected_italics, italics_replaced);
    }
}

pub fn format(input: &str, formatter: Formatter) -> Cow<str> {
    formatter.expression.replace_all(input, formatter.replacement)
}

pub struct Formatter {
    expression: Regex,
    replacement: &'static str,
}

impl Formatter {
    pub fn new(pattern: &str, multi_line: bool, replacement: &'static str) -> Formatter {
        let mut builder: RegexBuilder = RegexBuilder::new(pattern);
        builder.multi_line(multi_line);
        let expression: Regex = builder.build().expect("Could not create Regex from provided pattern");
        Formatter {
            expression,
            replacement
        }
    }
}

pub struct H1Formatter;

impl H1Formatter {
    pub fn new() -> Formatter {
        Formatter::new(r"^# (?P<title>.+)$", true, "<h1>${title}</h1>")
    }
}

pub struct H2Formatter;

impl H2Formatter {
    pub fn new() -> Formatter {
        Formatter::new(r"^## (?P<title>.+)$", true, "<h2>${title}</h2>")
    }
}

pub struct ItalicsFormatter;

impl ItalicsFormatter {
    pub fn new() -> Formatter {
        Formatter::new(r"\b_(?P<contents>.+)_\b", false, "<i>${contents}</i>")
    }
}