//! Hello-world snippet for `ProgrammingLanguageName::ReactNative`.
//!
//! Imported from leachim6/hello-world (`r/React Native.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/React%20Native.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ReactNative => "React Native",
    versions: [ReactNative => "React Native"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ReactNative,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ReactNative,
            syntax: concat!(
                r#"import React from "react";
import { Text, View } from "react-native";

export default function HelloWorld() {
    return (
        <View>
            <Text>"#,
                output_for_hello_world!(),
                r"</Text>
        </View>
    );
}"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" React "),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""react""#),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (TokenKind::Plain, r" Text"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" View "),
                (TokenKind::Punctuation, r"}"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" "),
                (TokenKind::String, r#""react-native""#),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"export"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"default"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"HelloWorld"),
                (TokenKind::Punctuation, r"()"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"return"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::Plain,
                    r"
        ",
                ),
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"View"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
            ",
                ),
                (TokenKind::Keyword, r"<"),
                (TokenKind::Plain, r"Text"),
                (TokenKind::Keyword, r">"),
                (TokenKind::Plain, output_for_hello_world!()),
                (TokenKind::Keyword, r"<"),
                (
                    TokenKind::String,
                    r"/Text>
        </",
                ),
                (TokenKind::Plain, r"View"),
                (TokenKind::Keyword, r">"),
                (
                    TokenKind::Plain,
                    r"
    );
}",
                ),
            ],
        }],
    };
