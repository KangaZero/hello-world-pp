//! Hello-world snippet for `ProgrammingLanguageName::React360`.
//!
//! Imported from leachim6/hello-world (`r/React360.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/React360.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: React360 => "React360",
    versions: [React360 => "React360"],
    extensions: [],
}

const REACT360: &str = concat!(
    r#"import React from "react";
import { AppRegistry, Text, View, StyleSheet } from "react-360";

export default class hello_world extends React.Component {
    render() {
        return (
            <View style={styles.panel}>
                <View style={styles.greetingBox}>
                    <Text style={styles.greeting}>"#,
    output_for_hello_world!(),
    r##"</Text>
                </View>
            </View>
        );
    }
}

const styles = StyleSheet.create({
    panel: {
        // Fill the entire surface
        width: 1000,
        height: 600,
        backgroundColor: "rgba(255, 255, 255, 0.4)",
        justifyContent: "center",
        alignItems: "center",
    },
    greetingBox: {
        padding: 20,
        backgroundColor: "#000000",
        borderColor: "#639dda",
        borderWidth: 2,
    },
    greeting: {
        fontSize: 30,
    },
});

AppRegistry.registerComponent("hello_world", () => hello_world);"##
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::React360,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::React360,
            syntax: REACT360,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REACT360,
                    [
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
                        (TokenKind::Plain, r" AppRegistry"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" Text"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" View"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" StyleSheet "),
                        (TokenKind::Punctuation, r"}"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""react-360""#),
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
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" hello_world "),
                        (TokenKind::Keyword, r"extends"),
                        (TokenKind::Plain, r" React"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Component "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"render"),
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
                        (TokenKind::Plain, r"View style"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r"styles.panel"),
                        (TokenKind::Punctuation, r"}"),
                        (TokenKind::Keyword, r">"),
                        (
                            TokenKind::Plain,
                            r"
                ",
                        ),
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"View style"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r"styles.greetingBox"),
                        (TokenKind::Punctuation, r"}"),
                        (TokenKind::Keyword, r">"),
                        (
                            TokenKind::Plain,
                            r"
                    ",
                        ),
                        (TokenKind::Keyword, r"<"),
                        (TokenKind::Plain, r"Text style"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r"styles.greeting"),
                        (TokenKind::Punctuation, r"}"),
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
            ",
                        ),
                        (TokenKind::Keyword, r"<"),
                        (
                            TokenKind::String,
                            r##"/View>
        );
    }
}

const styles = StyleSheet.create({
    panel: {
        // Fill the entire surface
        width: 1000,
        height: 600,
        backgroundColor: "rgba(255, 255, 255, 0.4)",
        justifyContent: "center",
        alignItems: "center",
    },
    greetingBox: {
        padding: 20,
        backgroundColor: "#000000",
        borderColor: "#639dda",
        borderWidth: 2,
    },
    greeting: {
        fontSize: 30,
    },
});

AppRegistry.registerComponent("hello_world", () => hello_world);"##,
                        ),
                    ],
                )
            },
        }],
    };
