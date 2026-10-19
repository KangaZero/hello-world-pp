//! Hello-world snippet for `ProgrammingLanguageName::Lazyk`.
//!
//! Imported from leachim6/hello-world (`l/LazyK.lazy` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Lazyk => "LazyK",
    versions: [Lazyk => "LazyK"],
    extensions: [Lazy => "lazy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lazyk,
        extensions: &[FileExtension::Lazy],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lazyk,
            syntax: r"`k``s``si`k``s`k```sii``s``s`kski``s``s`ksk``s``s`ksk```s``siii``s``s`kski`k``s``si`k``s``s`ksk```s``s`ksk
i``s`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k
``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski``s``s`ksk`
`s`k``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k````s``s`ksk```s``siii``s``s`kski`s``s`ksk```sii``s``
s`ksk``s``s`kski`k``s``si`k``s`k``s``s`kski```s``siii``s``s`kski`k``s``si`k``s`k``s``s`ksk``s`k``s``s`kski
``s``s`ksk``s``s`kski``s``s`ksk```s``siii``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski``s``s`ksk``s`k``
s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k``s`k``s``s`kski``s``s`ksk``s`k``s``s`kski``s``s`ksk```sii`
`s``s`ksk``s``s`kski`k``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k```s``s`kski``s
`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski```s``siii``s``s`kski`k``s``s
i`k``s`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k```sii```sii``s``s`kski`k```sii```sii``s``s`kski",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"`k``s``si`k``s`k```sii``s``s`kski``s``s`ksk``s``s`ksk```s``siii``s``s`kski`k``s``si`k``s``s`ksk```s``s`ksk
i``s`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k
``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski``s``s`ksk`
`s`k``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k````s``s`ksk```s``siii``s``s`kski`s``s`ksk```sii``s``
s`ksk``s``s`kski`k``s``si`k``s`k``s``s`kski```s``siii``s``s`kski`k``s``si`k``s`k``s``s`ksk``s`k``s``s`kski
``s``s`ksk``s``s`kski``s``s`ksk```s``siii``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski``s``s`ksk``s`k``
s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k``s`k``s``s`kski``s``s`ksk``s`k``s``s`kski``s``s`ksk```sii`
`s``s`ksk``s``s`kski`k``s``si`k``s`k```sii``s``s`kski```sii``s``s`ksk``s``s`kski`k``s``si`k```s``s`kski``s
`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k``s``s`ksk``s`k``s``s`kski```s``siii``s``s`kski`k``s``s
i`k``s`k``s``s`kski``s``s`ksk```sii``s``s`kski`k``s``si`k```sii```sii``s``s`kski`k```sii```sii``s``s`kski",
            )],
        }],
    };
