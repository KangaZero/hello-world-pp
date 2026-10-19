//! Hello-world snippet for `ProgrammingLanguageName::Decimal`.
//!
//! Imported from leachim6/hello-world (`d/Decimal.dec` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Decimal => "Decimal",
    versions: [Decimal => "Decimal"],
    extensions: [Dec => "dec"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Decimal,
        extensions: &[FileExtension::Dec],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Decimal,
            syntax: r" 13072101108108111044032087111114108100033010D301
 ;13                                               push string
 ;  072                                            'H'
 ;     101                                         'e'
 ;        108                                      'l'
 ;           108                                   'l'
 ;              111                                'o'
 ;                 044                             ','
 ;                    032                          ' '
 ;                       087                       'W'
 ;                          111                    'o'
 ;                             114                 'r'
 ;                                108              'l'
 ;                                   100           'd'
 ;                                      033        '!'
 ;                                         010     '\n'
 ;                                            D    end string
 ;                                             301 print",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r" 13072101108108111044032087111114108100033010D301
 ;13                                               push string
 ;  072                                            'H'
 ;     101                                         'e'
 ;        108                                      'l'
 ;           108                                   'l'
 ;              111                                'o'
 ;                 044                             ','
 ;                    032                          ' '
 ;                       087                       'W'
 ;                          111                    'o'
 ;                             114                 'r'
 ;                                108              'l'
 ;                                   100           'd'
 ;                                      033        '!'
 ;                                         010     '\n'
 ;                                            D    end string
 ;                                             301 print",
            )],
        }],
    };
