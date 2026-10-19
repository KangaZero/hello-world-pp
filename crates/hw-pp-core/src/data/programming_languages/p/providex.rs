//! Hello-world snippet for `ProgrammingLanguageName::Providex`.
//!
//! Imported from leachim6/hello-world (`p/ProvideX.vim` at `a152253`),
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
    language: Providex => "ProvideX",
    versions: [Providex => "ProvideX"],
    extensions: [Vim => "vim"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Providex,
        extensions: &[FileExtension::Vim],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Providex,
            syntax: concat!(
                r#"begin  
   print ""#,
                output_for_hello_world!(),
                r#"" 
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"begin  
   print ""#,
                    output_for_hello_world!(),
                    r#"" 
end"#
                ),
            )],
        }],
    };
