//! Hello-world snippet for `ProgrammingLanguageName::Spss`.
//!
//! Imported from leachim6/hello-world (`s/SPSS.spss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SPSS.spss>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Spss => "SPSS",
    versions: [Spss => "SPSS"],
    extensions: [Spss => "spss"],
}

const SPSS: &str = concat!(
    r#"BEGIN PROGRAM.
print ""#,
    output_for_hello_world!(),
    r#""
END PROGRAM."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spss,
        extensions: &[FileExtension::Spss],
        categories: &[Scientific],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spss,
            syntax: SPSS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPSS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"BEGIN PROGRAM.
print ""#,
                            output_for_hello_world!(),
                            r#""
END PROGRAM."#
                        ),
                    )],
                )
            },
        }],
    };
