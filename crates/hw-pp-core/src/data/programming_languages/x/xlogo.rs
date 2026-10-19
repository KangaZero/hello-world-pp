//! Hello-world snippet for `ProgrammingLanguageName::Xlogo`.
//!
//! Imported from leachim6/hello-world (`x/Xlogo.lgo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Xlogo => "Xlogo",
    versions: [Xlogo => "Xlogo"],
    extensions: [Lgo => "lgo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xlogo,
        extensions: &[FileExtension::Lgo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xlogo,
            syntax: r"to HelloWorld
 resetall
 hideturtle
 fd 20 left 180
 fd 40 left 180
 fd 20 right 90
 fd 20 left 90
 fd 20 left 180
 fd 40 left 90
 fd 20 left 90
 fd 20 right 90
 fd 20 right 90
 fd 10 right 90
 fd 20 left 90
 fd 10 left 90
 fd 30 left 90
 fd 40 left 180
 fd 40 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 60 left 90
 fd 40 left 180
 fd 40 left 90
 fd 20 left 90
 fd 20 left 180
 fd 20 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 40 left 90
 fd 20 right 90
 fd 20 right 90
 fd 5  left 90  
 fd 5  left 90  
 fd 25 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40
end",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"to HelloWorld
 resetall
 hideturtle
 fd 20 left 180
 fd 40 left 180
 fd 20 right 90
 fd 20 left 90
 fd 20 left 180
 fd 40 left 90
 fd 20 left 90
 fd 20 right 90
 fd 20 right 90
 fd 10 right 90
 fd 20 left 90
 fd 10 left 90
 fd 30 left 90
 fd 40 left 180
 fd 40 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 60 left 90
 fd 40 left 180
 fd 40 left 90
 fd 20 left 90
 fd 20 left 180
 fd 20 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 40 left 90
 fd 20 right 90
 fd 20 right 90
 fd 5  left 90  
 fd 5  left 90  
 fd 25 left 180
 fd 40 left 90
 fd 40 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 20 left 90
 fd 40 left 180
 fd 40
end",
            )],
        }],
    };
