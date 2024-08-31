#![no_std]

extern crate alloc;

use alloc::{str::Chars, string::String};

fn next_char_is_whitespace(chars: &Chars<'_>) -> bool {
    chars.clone().next().map_or(false, |c| c.is_whitespace())
}

fn next_chars_lowercase_matches(chars: &Chars<'_>, expected: &str) -> bool {
    chars.clone().take(expected.len()).map(|c| c.to_ascii_lowercase()).eq(expected.chars())
}

fn consume_html_quote(chars: &mut Chars<'_>, quote: char) {
    for c in chars {
        if c == quote {
            break;
        }
    }
}

fn consume_php_quote(chars: &mut Chars<'_>, quote: char) {
    let mut last_char = quote;

    for c in chars {
        if c == quote && last_char != '\\' {
            break;
        }

        last_char = c;
    }
}

fn consume_parenthesis(chars: &mut Chars<'_>) {
    let mut count = 1;

    for c in chars {
        match c {
            '(' => count += 1,
            ')' if count > 1 => count -= 1,
            ')' => break,
            _ => {}
        }
    }
}

fn consume_php(chars: &mut Chars<'_>) {
    let mut last_char = '?';

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                consume_parenthesis(chars);
                last_char = ')';
            }

            '>' if last_char == '?' => {
                break;
            }

            '\'' | '"' if last_char != '\\' => {
                consume_php_quote(chars, c);
                last_char = c;
            }

            _ => last_char = c,
        }
    }
}

fn consume_comment(chars: &mut Chars<'_>) {
    while let Some(c) = chars.next() {
        if c == '-' && next_chars_lowercase_matches(chars, "->") {
            chars.next();
            chars.next();
            break;
        }
    }
}

fn consume_xml(chars: &mut Chars<'_>) {
    let mut last_char = 'l';

    while let Some(c) = chars.next() {
        match c {
            '>' if last_char != '-' => break,

            '<' => {
                consume_tag(chars);
                last_char = '>';
            }

            '\'' | '"' => {
                consume_html_quote(chars, c);
                last_char = c;
            }

            _ => last_char = c,
        }
    }
}

fn consume_tag(chars: &mut Chars<'_>) {
    let mut last_char = '<';

    while let Some(c) = chars.next() {
        match c {
            '>' => break,

            '<' => {
                consume_tag(chars);
                last_char = '>';
            }

            '\'' | '"' => {
                consume_html_quote(chars, c);
                last_char = c;
            }

            '?' if last_char == '<' => {
                if next_chars_lowercase_matches(chars, "xml") {
                    chars.next();
                    chars.next();
                    chars.next();
                    consume_xml(chars);
                } else {
                    consume_php(chars);
                }

                break;
            }

            '!' if last_char == '<' && next_chars_lowercase_matches(chars, "--") => {
                chars.next();
                chars.next();
                consume_comment(chars);
                break;
            }

            _ => last_char = c,
        }
    }
}

/// Strips HTML and PHP tags from the input string.
///
/// This function tries to return a string with all NULL bytes, HTML and PHP tags stripped from a given string. It mimics the behavior of [PHP's strip_tags() function](https://www.php.net/manual/en/function.strip-tags.php).
pub fn strip_tags(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        match c {
            '\0' => continue,
            '<' if !next_char_is_whitespace(&chars) => consume_tag(&mut chars),
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::strip_tags;

    #[test]
    fn it_works() {
        // My own tests
        assert_eq!(strip_tags("Hello, world!"), "Hello, world!");
        assert_eq!(strip_tags("<>"), "");
        assert_eq!(strip_tags("<>Hello"), "Hello");
        assert_eq!(strip_tags("<<>Hello>World"), "World");
        assert_eq!(strip_tags("< <>Hello>World"), "< Hello>World");

        // strip_tags() function
        assert_eq!(strip_tags("NEAT <? cool < blah ?> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("NEAT <? cool > blah ?> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("NEAT <!-- cool < blah --> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("NEAT <!-- cool > blah --> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("NEAT <? echo \\\"\\\\\"\\\"?> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("NEAT <? echo '\\''?> STUFF"), "NEAT  STUFF");
        assert_eq!(strip_tags("TESTS ?!!?!?!!!?!!"), "TESTS ?!!?!?!!!?!!");

        // Test strip_tags() function : basic functionality - with array argument
        assert_eq!(strip_tags("<p>foo <b>bar</b> <a href=\"#\">foobar</a></p>"), "foo bar foobar");

        // Test strip_tags() function : basic functionality - with default arguments
        assert_eq!(strip_tags("<html>hello</html>"), "hello");
        assert_eq!(strip_tags("<html>hello</html>"), "hello");
        assert_eq!(strip_tags("<?php echo hello ?>"), "");
        assert_eq!(strip_tags("<?php echo hello ?>"), "");
        assert_eq!(strip_tags("<? echo hello ?>"), "");
        assert_eq!(strip_tags("<? echo hello ?>"), "");
        assert_eq!(strip_tags("<% echo hello %>"), "");
        assert_eq!(strip_tags("<% echo hello %>"), "");
        assert_eq!(strip_tags("<script language=\"PHP\"> echo hello </script>"), " echo hello ");
        assert_eq!(strip_tags("<script language=\\\"PHP\\\"> echo hello </script>"), " echo hello ");
        assert_eq!(strip_tags("<html><b>hello</b><p>world</p></html>"), "helloworld");
        assert_eq!(strip_tags("<html><b>hello</b><p>world</p></html>"), "helloworld");
        assert_eq!(strip_tags("<html><!-- COMMENT --></html>"), "");
        assert_eq!(strip_tags("<html><!-- COMMENT --></html>"), "");

        // Test strip_tags() function : obscure values within attributes
        assert_eq!(strip_tags("hello <img title=\"<\"> world"), "hello  world");
        assert_eq!(strip_tags("hello <img title=\">\"> world"), "hello  world");
        assert_eq!(strip_tags("hello <img title=\">_<\"> world"), "hello  world");
        assert_eq!(strip_tags("hello <img title='>_<'> world"), "hello  world");

        // Test strip_tags() function : usage variations - binary safe checking
        assert_eq!(strip_tags("<html> I am html string </html>\0<?php I am php string ?>"), " I am html string ");
        assert_eq!(strip_tags("<html> I am html string\0 </html><?php I am php string ?>"), " I am html string ");
        assert_eq!(strip_tags("<a>I am html string</a>"), "I am html string");
        assert_eq!(strip_tags("<html>I am html string</html>1000001<?php I am php string?>"), "I am html string1000001");
    }

    // Bug #21453 (handling of non-encoded <)
    #[test]
    fn bug_21453() {
        let input = "
<table>
    <tr><td>first cell before < first cell after</td></tr>
    <tr><td>second cell before < second cell after</td></tr>
</table>";

        let expected = "

    first cell before < first cell after
    second cell before < second cell after
";

        assert_eq!(strip_tags(input), expected);
    }

    // Bug #21744 (strip_tags misses exclamation marks in alt text)
    #[test]
    fn bug_21744() {
        let input = "
<a href=\"test?test\\!!!test\">test</a>
<!-- test -->";

        let expected = "
test
";

        assert_eq!(strip_tags(input), expected);
    }

    // Bug #23650 (strip_tags() removes hyphens)
    #[test]
    fn bug_23650() {
        let input = "
1:<!-- abc -  -->
2:<!doctype -- >
3:
4:<abc - def>
5:abc - def
6:</abc>
";

        let expected = "
1:
2:
3:
4:
5:abc - def
6:
";

        assert_eq!(strip_tags(input), expected);
    }

    // Bug #40432 (strip_tags() fails with greater than in attribute)
    #[test]
    fn bug_40432() {
        let input = "<span title=\"test > all\">this</span>";

        assert_eq!(strip_tags(input), "this");
    }

    // Bug #40637 (strip_tags() does not handle single quotes correctly)
    #[test]
    fn bug_40637() {
        let input = "<span title=\"Bug ' Trigger\">Text</span>";

        assert_eq!(strip_tags(input), "Text");
    }

    // Bug #40704 (strip_tags() does not handle single quotes correctly)
    #[test]
    fn bug_40704() {
        let input = "<div>Bug ' Trigger</div> Missing Text";

        assert_eq!(strip_tags(input), "Bug ' Trigger Missing Text");
    }

    // Bug #45485 (strip_tags and <?XML tag)
    #[test]
    fn bug_45485() {
        assert_eq!(
            strip_tags("This text is shown <?XML:NAMESPACE PREFIX = ST1 /><b>This Text disappears</b>"),
            "This text is shown This Text disappears"
        );

        assert_eq!(
            strip_tags("This text is shown <?xml:NAMESPACE PREFIX = ST1 /><b>This Text disappears</b>"),
            "This text is shown This Text disappears"
        );
    }

    // Bug #46578 (strip_tags() does not honor end-of-comment when it encounters a single quote)
    #[test]
    fn bug_46578() {
        assert_eq!(strip_tags("<!-- testing I\'ve been to mars -->foobar"), "foobar");
        assert_eq!(strip_tags("<a alt=\"foobar\">foo<!-- foo! --></a>bar"), "foobar");
        assert_eq!(strip_tags("<a alt=\"foobar\"/>foo<?= foo! /* <!-- \"cool\" --> */ ?>bar"), "foobar");
        assert_eq!(strip_tags("< ax"), "< ax");
        assert_eq!(strip_tags("<! a>"), "");
        assert_eq!(strip_tags("<? ax"), "");
    }

    // Bug #50847 (strip_tags() removes all tags greater than 1023 bytes long)
    #[test]
    fn bug_50847() {
        let input = format!("<param value=\"{}\" />", "a".repeat(2048));

        assert_eq!(strip_tags(&input), "");
    }

    // Bug #53319 (Strip_tags() may strip '<br />' incorrectly)
    #[test]
    fn bug_53319() {
        assert_eq!(strip_tags("<br /><br  />USD<input type=\"text\"/><br/>CDN<br><input type=\"text\" />"), "USDCDN");
    }

    // Bug #70720 (strip_tags() doesn't handle "xml" correctly)
    #[test]
    fn bug_70720() {
        assert_eq!(strip_tags("<?php $dom->test(); ?> this is a test"), " this is a test");
        assert_eq!(strip_tags("<?php $xml->test(); ?> this is a test"), " this is a test");
        assert_eq!(strip_tags("<?xml $xml->test(); ?> this is a test"), " this is a test");
        assert_eq!(strip_tags("<span class=sf-dump-> this is a test</span>"), " this is a test");
    }

    // Bug #78003 (strip_tags output change since PHP 7.3)
    #[test]
    fn bug_78003() {
        assert_eq!(strip_tags("<foo<>bar>"), "");
        assert_eq!(strip_tags("<foo<!>bar>"), "");
        assert_eq!(strip_tags("<foo<?>bar>"), "");
    }

    // Bug #78346 (strip_tags no longer handling nested php tags)
    #[test]
    fn bug_78346() {
        assert_eq!(strip_tags("<?= '<?= 1 ?>' ?>2"), "2");
    }

    // Bug #79099 (OOB read in php_strip_tags_ex)
    #[test]
    fn bug_79099() {
        assert_eq!(strip_tags("<?\n\"\n"), "");
        assert_eq!(strip_tags("<\0\n!\n"), "");
        assert_eq!(strip_tags("<\0\n?\n"), "");
    }
}
