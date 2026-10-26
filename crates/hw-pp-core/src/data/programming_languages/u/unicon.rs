//! Hello-world snippet for `ProgrammingLanguageName::Unicon`.
//!
//! Imported from leachim6/hello-world (`u/Unicon.icn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Unicon.icn>
//! - <https://www.wikidata.org/wiki/Q3279945> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Icon, Snobol},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Unicon => "Unicon",
    versions: [Unicon => "Unicon"],
    extensions: [],
}

const UNICON: &str = concat!(
    r#"procedure main()
    write(""#,
    output_for_hello_world!(),
    r#"")
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Unicon,
        extensions: &[],
        categories: &[Scripting],
        families: &[Icon, Snobol],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Unicon,
            syntax: UNICON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNICON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"procedure main()
    write(""#,
                            output_for_hello_world!(),
                            r#"")
end"#
                        ),
                    )],
                )
            },
        }],
    };
