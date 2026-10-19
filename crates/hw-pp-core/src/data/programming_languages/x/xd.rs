//! Hello-world snippet for `ProgrammingLanguageName::XD`.
//!
//! Imported from leachim6/hello-world (`x/X D.xd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/X%20D.xd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: XD => "X D",
    versions: [XD => "X D"],
    extensions: [Xd => "xd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XD,
        extensions: &[FileExtension::Xd],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XD,
            syntax: r";-~~~~~> ;P ;~~> ;P ;------> ;-P ;--> ;P ;D ;~~---> ;P ;| ;~~< ;----> ;P
;~---------> ;P ;--> ;P ;-----< ;P ;-------< ;P ;D ;> ;P ;~< ;-------< ;P",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r";-~~~~~> ;P ;~~> ;P ;------> ;-P ;--> ;P ;D ;~~---> ;P ;| ;~~< ;----> ;P
;~---------> ;P ;--> ;P ;-----< ;P ;-------< ;P ;D ;> ;P ;~< ;-------< ;P",
            )],
        }],
    };
