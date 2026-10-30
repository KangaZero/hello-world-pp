//! Hello-world snippet for `ProgrammingLanguageName::Vimscript`.
//!
//! Imported from leachim6/hello-world (`v/VimScript.vim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/v/VimScript.vim>
//! - <https://www.wikidata.org/wiki/Q7931258> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Vimscript => "VimScript",
    versions: [Vimscript => "VimScript"],
    extensions: [],
}

const VIMSCRIPT: &str = concat!(r#"echo ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vimscript,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vimscript,
            syntax: VIMSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VIMSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"echo ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
