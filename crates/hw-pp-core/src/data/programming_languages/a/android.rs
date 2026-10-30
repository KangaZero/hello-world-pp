//! Hello-world snippet for `ProgrammingLanguageName::Android`.
//!
//! Imported from leachim6/hello-world (`a/Android.java` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Android.java>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Android => "Android",
    versions: [Android => "Android"],
    extensions: [],
}

const ANDROID: &str = concat!(
    r#"package com.example.helloworld;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class HelloWorld extends Activity {

   @Override
   public void onCreate(Bundle savedInstanceState) {
       super.onCreate(savedInstanceState);
       TextView tv = new TextView(this);
       tv.setText(""#,
    output_for_hello_world!(),
    r#"");
       setContentView(tv);
   }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Android,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Android,
            syntax: ANDROID,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANDROID,
                    [
                        (TokenKind::Keyword, r"package"),
                        (TokenKind::Plain, r" com.example.helloworld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" android.app"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Activity"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" android.os"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Bundle"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" android.widget"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"TextView"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"class"),
                        (TokenKind::Plain, r" HelloWorld "),
                        (TokenKind::Keyword, r"extends"),
                        (TokenKind::Plain, r" Activity "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"

   ",
                        ),
                        (TokenKind::Punctuation, r"@"),
                        (
                            TokenKind::Plain,
                            r"Override
   ",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"onCreate"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"Bundle savedInstanceState"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
       super",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"onCreate"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"savedInstanceState"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
       TextView tv ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"new"),
                        (TokenKind::Plain, r" TextView"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"this"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
       tv",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"setText"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
       ",
                        ),
                        (TokenKind::Function, r"setContentView"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"tv"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Punctuation, r"}"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
