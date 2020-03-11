/// This module is resposible for escape logic in influxdb line protocol
///
/// [External
/// doc](https://v2.docs.influxdata.com/v2.0/reference/syntax/line-protocol/#special-characters)

#[inline]
fn escape<P>(is_escape_char: P, s: &str) -> String
where
    P: Fn(char) -> bool,
{
    let s_len = s.len();
    let begin = s.find(|c| is_escape_char(c)).unwrap_or(s_len);

    // we add extra bytes to prevent unnecessary copy
    let mut escaped_string = String::with_capacity(s_len + 8);
    escaped_string += &s[..begin];
    for c in s[begin..].chars() {
        if is_escape_char(c) {
            escaped_string.push('\\');
            escaped_string.push(c);
        } else {
            escaped_string.push(c);
        }
    }
    escaped_string
}

#[inline]
fn escape_comma_equal_space(c: char) -> bool {
    match c {
        '=' | ',' | ' ' => true,
        _c => false,
    }
}

#[inline]
pub fn tag_key(s: &str) -> String {
    escape(escape_comma_equal_space, s)
}

#[inline]
pub fn field_key(s: &str) -> String {
    escape(escape_comma_equal_space, s)
}

#[inline]
pub fn tag_value(s: &str) -> String {
    escape(escape_comma_equal_space, s)
}

#[inline]
pub fn field_value(s: &str) -> String {
    escape(
        |c| match c {
            '"' | '\\' => true,
            _c => false,
        },
        s,
    )
}

#[inline]
pub fn measurement(s: &str) -> String {
    escape(
        |c| match c {
            ',' | ' ' => true,
            _c => false,
        },
        s,
    )
}

#[cfg(all(feature = "nightly", test))]
mod bench {
    const NO_ESCAPE: &str = r#"Abcdefghijklmnouódsałπ≠²³4tonżðąq"#;
    const TO_ESCAPE: &str = r#"asdddas\  =d =das=sddsałπ≠²³4tonż"#;
    use super::*;
    extern crate test;
    use regex::Regex;

    #[inline]
    fn escape_with_replace(s: &String) -> String {
        s.replace("=", r#"\="#)
            .replace(",", r#"\,"#)
            .replace(" ", r#"\ "#)
    }

    #[inline]
    fn escape_with_replace2(s: &String) -> String {
        s.replace('=', r#"\="#)
            .replace(',', r#"\,"#)
            .replace(' ', r#"\ "#)
    }

    #[inline]
    fn escape_with_contains_replace(s: String) -> String {
        let s = if s.contains("=") {
            s.replace("=", r#"\="#)
        } else {
            s
        };
        let s = if s.contains(",") {
            s.replace(",", r#"\,"#)
        } else {
            s
        };

        if s.contains(" ") {
            s.replace(" ", r#"\ "#)
        } else {
            s
        }
    }

    #[inline]
    fn escape_find_push_uo(s: &String) -> String {
        let escape = s
            .find(|c| match c {
                '=' | ',' | ' ' => true,
                _ => false,
            })
            .map(|_| true)
            .unwrap_or(false);

        if escape {
            let mut escaped_string = String::with_capacity(s.len() + 8);
            for c in s.chars() {
                match c {
                    '=' => escaped_string.push_str(r#"\="#),
                    ',' => escaped_string.push_str(r#"\,"#),
                    ' ' => escaped_string.push_str(r#"\ "#),
                    c => escaped_string.push(c),
                }
            }
            escaped_string
        } else {
            s.clone()
        }
    }

    #[inline]
    fn escape_find_push(s: &String) -> String {
        let opt_begin = s.find(|c| match c {
            '=' | ',' | ' ' => true,
            _ => false,
        });

        // begin contains position where first item was found
        if let Some(begin) = opt_begin {
            let mut escaped_string = String::with_capacity(s.len() + 8);
            escaped_string.push_str(&s[..begin]); // from 0 to first item(without it)
            for c in s[begin..].chars() {
                // skip copied chars
                match c {
                    '=' => escaped_string.push_str(r#"\="#),
                    ',' => escaped_string.push_str(r#"\,"#),
                    ' ' => escaped_string.push_str(r#"\ "#),
                    c => escaped_string.push(c),
                }
            }
            escaped_string
        } else {
            s.clone()
        }
    }

    #[inline]
    fn escape_find_push2(s: &String) -> String {
        let s_len = s.len();
        let is_escape_char = move |c| match c {
            '=' | ',' | ' ' => true,
            _c => false,
        };
        let begin = s.find(is_escape_char).unwrap_or(s_len);

        // we add extra bytes to prevent unnecessary copy
        let mut escaped_string = String::with_capacity(s_len + 8);
        escaped_string += &s[..begin];
        for c in s[begin..].chars() {
            if is_escape_char(c) {
                escaped_string.push('\\');
                escaped_string.push(c);
            } else {
                escaped_string.push(c);
            }
        }
        escaped_string
    }

    #[bench]
    fn no_escpae_regex(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        let re = Regex::new("[, =]").unwrap();
        b.iter(|| re.replace_all(&s, r#"\$0"#).to_string())
    }

    #[bench]
    fn to_escape_regex(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        let re = Regex::new("[, =]").unwrap();
        b.iter(|| re.replace_all(&s, r#"\$0"#).to_string())
    }

    #[bench]
    fn no_escape_std_replace(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_with_replace(&s))
    }

    #[bench]
    fn to_escape_std_replace(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_with_replace(&s))
    }

    #[bench]
    fn no_escape_std_replace2(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_with_replace2(&s))
    }

    #[bench]
    fn to_escape_std_replace2(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_with_replace2(&s))
    }

    #[bench]
    fn no_escape_std_contains_replace(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_with_contains_replace(s.clone()))
    }

    #[bench]
    fn to_escape_std_contains_replace(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_with_contains_replace(s.clone()))
    }

    #[bench]
    fn no_escape_std_find_push_uo(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_find_push_uo(&s))
    }

    #[bench]
    fn to_escape_std_find_push_uo(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_find_push_uo(&s))
    }

    #[bench]
    fn no_escape_find_push(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_find_push(&s))
    }

    #[bench]
    fn to_escape_find_push(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_find_push(&s))
    }

    #[bench]
    fn no_escape_find_push2(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| escape_find_push2(&s))
    }

    #[bench]
    fn to_escape_find_push2(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| escape_find_push2(&s))
    }

    #[bench]
    fn no_escape_general(b: &mut test::Bencher) {
        let s = String::from(NO_ESCAPE);
        b.iter(|| {
            escape(
                |c| match c {
                    '=' | ',' | ' ' => true,
                    _c => false,
                },
                &s,
            )
        })
    }

    #[bench]
    fn to_escape_general(b: &mut test::Bencher) {
        let s = String::from(TO_ESCAPE);
        b.iter(|| {
            escape(
                |c| match c {
                    '=' | ',' | ' ' => true,
                    _c => false,
                },
                &s,
            )
        })
    }
}
