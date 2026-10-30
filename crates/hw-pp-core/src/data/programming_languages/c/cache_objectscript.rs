//! Hello-world snippet for `ProgrammingLanguageName::CacheObjectscript`.
//!
//! Imported from leachim6/hello-world (`c/Cache ObjectScript.mac` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cache%20ObjectScript.mac>
//! - <https://www.wikidata.org/wiki/Q5016025> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: CacheObjectscript => "Cache ObjectScript",
    versions: [CacheObjectscript => "Cache ObjectScript"],
    extensions: [Mac => "mac"],
}

const CACHE_OBJECTSCRIPT: &str = concat!(
    r#"HelloWorld  ;
  Write ""#,
    output_for_hello_world!(),
    r#""
  Quit"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CacheObjectscript,
        extensions: &[FileExtension::Mac],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CacheObjectscript,
            syntax: CACHE_OBJECTSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CACHE_OBJECTSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"HelloWorld  ;
  Write ""#,
                            output_for_hello_world!(),
                            r#""
  Quit"#
                        ),
                    )],
                )
            },
        }],
    };
