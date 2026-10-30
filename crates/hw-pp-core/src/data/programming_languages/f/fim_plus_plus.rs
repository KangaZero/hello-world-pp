//! Hello-world snippet for `ProgrammingLanguageName::FimPlusPlus`.
//!
//! Imported from leachim6/hello-world (`f/Fim++.fimpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fim%2B%2B.fimpp>
//! - <https://esolangs.org/wiki/FiM%2B%2B> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: FimPlusPlus => "Fim++",
    versions: [FimPlusPlus => "Fim++"],
    extensions: [Fimpp => "fimpp"],
}

const FIM_PLUS_PLUS: &str = concat!(
    r"Dear Princess Celestia: ",
    output_for_hello_world!(),
    r"

Today I learned how to say ",
    output_for_hello_world!(),
    r#"
I said ""#,
    output_for_hello_world!(),
    r#"".
That's all about how to say "#,
    output_for_hello_world!(),
    r".

Your faithful student, Twilight Sparkle. "
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FimPlusPlus,
        extensions: &[FileExtension::Fimpp],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FimPlusPlus,
            syntax: FIM_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FIM_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Dear Princess Celestia: ",
                            output_for_hello_world!(),
                            r"

Today I learned how to say ",
                            output_for_hello_world!(),
                            r#"
I said ""#,
                            output_for_hello_world!(),
                            r#"".
That's all about how to say "#,
                            output_for_hello_world!(),
                            r".

Your faithful student, Twilight Sparkle. "
                        ),
                    )],
                )
            },
        }],
    };
