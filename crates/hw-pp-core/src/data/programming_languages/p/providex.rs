//! Hello-world snippet for `ProgrammingLanguageName::Providex`.
//!
//! Imported from leachim6/hello-world (`p/ProvideX.vim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/ProvideX.vim>
//! - <https://www.wikidata.org/wiki/Q3408297> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Providex => "ProvideX",
    versions: [Providex => "ProvideX"],
    extensions: [Vim => "vim"],
}

const PROVIDEX: &str = concat!(
    r#"begin  
   print ""#,
    output_for_hello_world!(),
    r#"" 
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Providex,
        extensions: &[FileExtension::Vim],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Providex,
            syntax: PROVIDEX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PROVIDEX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"begin  
   print ""#,
                            output_for_hello_world!(),
                            r#"" 
end"#
                        ),
                    )],
                )
            },
        }],
    };
