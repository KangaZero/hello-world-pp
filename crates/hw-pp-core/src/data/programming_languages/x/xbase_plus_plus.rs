//! Hello-world snippet for `ProgrammingLanguageName::XbasePlusPlus`.
//!
//! Imported from leachim6/hello-world (`x/XBase++.prg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XBase%2B%2B.prg>
//! - <https://www.wikidata.org/wiki/Q1753410> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: XbasePlusPlus => "XBase++",
    versions: [XbasePlusPlus => "XBase++"],
    extensions: [],
}

const XBASE_PLUS_PLUS: &str = concat!(
    r#"func Main()
    Qout(""#,
    output_for_hello_world!(),
    r#"")
return 1"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XbasePlusPlus,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XbasePlusPlus,
            syntax: XBASE_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XBASE_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"func Main()
    Qout(""#,
                            output_for_hello_world!(),
                            r#"")
return 1"#
                        ),
                    )],
                )
            },
        }],
    };
