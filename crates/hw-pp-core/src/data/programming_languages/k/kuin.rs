//! Hello-world snippet for `ProgrammingLanguageName::Kuin`.
//!
//! Imported from leachim6/hello-world (`k/Kuin.kn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Kuin => "Kuin",
    versions: [Kuin => "Kuin"],
    extensions: [Kn => "kn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kuin,
        extensions: &[FileExtension::Kn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kuin,
            syntax: concat!(
                r#"func main()
  do cui@print(""#,
                output_for_hello_world!(),
                r#"\n")
end func"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"func main()
  do cui@print(""#,
                    output_for_hello_world!(),
                    r#"\n")
end func"#
                ),
            )],
        }],
    };
