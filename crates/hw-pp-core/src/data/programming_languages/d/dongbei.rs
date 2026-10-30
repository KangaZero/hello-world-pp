//! Hello-world snippet for `ProgrammingLanguageName::Dongbei`.
//!
//! Imported from leachim6/hello-world (`d/dongbei.dongbei` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/dongbei.dongbei>
//! - <https://esolangs.org/wiki/Dongbei> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dongbei => "dongbei",
    versions: [Dongbei => "dongbei"],
    extensions: [Dongbei => "dongbei"],
}

const DONGBEI: &str = concat!(r"唠唠：“", output_for_hello_world!(), r"”。 ");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dongbei,
        extensions: &[FileExtension::Dongbei],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dongbei,
            syntax: DONGBEI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DONGBEI,
                    [(
                        TokenKind::Plain,
                        concat!(r"唠唠：“", output_for_hello_world!(), r"”。 "),
                    )],
                )
            },
        }],
    };
