//! Hello-world snippet for `ProgrammingLanguageName::Matrix`.
//!
//! Imported from the Hello World Collection (`files/Matrix.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Matrix.htm>
//! - <http://helloworldcollection.de/#Matrix>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Matrix => "Matrix",
    versions: [Matrix => "Matrix"],
    extensions: [],
}

const MATRIX: &str = concat!(
    r"| ",
    output_for_hello_world!(),
    r#" in Matrix
 
System:Hello_World
 
    Type
        BaseType
            String
                Text : 12
 
    Matrix:Model:Application
        Silo:M1:Software_Application
            Realm:Analysis_Of_Application
                Domain:Friend
                    Entity:Greeting
                        State
                            Dataset:Hail
                                Text : Message
                            DynamicDataState:Display:Hail
                                StateAction
                                    Template
                                        <<|>><<#Message>>
                                        <<:>>
                                Transition
                                    EventState
                                        Wave -> Display
 
        Silo:M0:Real_World_Objects
            RealmObject:Objects_Of_Application
                DomainObject:Friend
                    EntityObject:Greeting
                        Object
                            | Identity | State
                            ID001      : Display
                        ObjectEvent
                            | Identity | Event | Message
                            ID001      : Wave  : ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Matrix,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Matrix,
            syntax: MATRIX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MATRIX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"| ",
                            output_for_hello_world!(),
                            r#" in Matrix
 
System:Hello_World
 
    Type
        BaseType
            String
                Text : 12
 
    Matrix:Model:Application
        Silo:M1:Software_Application
            Realm:Analysis_Of_Application
                Domain:Friend
                    Entity:Greeting
                        State
                            Dataset:Hail
                                Text : Message
                            DynamicDataState:Display:Hail
                                StateAction
                                    Template
                                        <<|>><<#Message>>
                                        <<:>>
                                Transition
                                    EventState
                                        Wave -> Display
 
        Silo:M0:Real_World_Objects
            RealmObject:Objects_Of_Application
                DomainObject:Friend
                    EntityObject:Greeting
                        Object
                            | Identity | State
                            ID001      : Display
                        ObjectEvent
                            | Identity | Event | Message
                            ID001      : Wave  : ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
