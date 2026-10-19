//! Hello-world snippet for `ProgrammingLanguageName::Dylan`.
//!
//! Imported from leachim6/hello-world (`d/Dylan.dl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dylan => "Dylan",
    versions: [Dylan => "Dylan"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dylan,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dylan,
            syntax: concat!(
                r#"define method main (#rest args)
  princ(""#,
                output_for_hello_world!(),
                r#"");
end;

main();"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"define method main (#rest args)
  princ(""#,
                    output_for_hello_world!(),
                    r#"");
end;

main();"#
                ),
            )],
        }],
    };
