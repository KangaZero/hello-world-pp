//! Hello-world snippet for `ProgrammingLanguageName::Peoplecode`.
//!
//! Imported from leachim6/hello-world (`p/PeopleCode` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PeopleCode>
//! - <https://www.wikidata.org/wiki/Q7165836> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, ObjectOriented};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Peoplecode => "PeopleCode",
    versions: [Peoplecode => "PeopleCode"],
    extensions: [],
}

const PEOPLECODE: &str = concat!(
    r#"MessageBox(0, "", 0, 0, ""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Peoplecode,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Peoplecode,
            syntax: PEOPLECODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PEOPLECODE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"MessageBox(0, "", 0, 0, ""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };
