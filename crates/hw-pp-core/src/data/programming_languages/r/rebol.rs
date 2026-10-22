//! Hello-world snippets for `ProgrammingLanguageName::Rebol`.
//!
//! Imported from leachim6/hello-world (`r/Rebol.r`, `r/Rebol.reb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rebol.r>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rebol.reb>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Rebol => "Rebol",
    versions: [RebolR => "Rebol (.r)", RebolReb => "Rebol (.reb)"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rebol,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RebolR,
                syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RebolReb,
                syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                )],
            },
        ],
    };
