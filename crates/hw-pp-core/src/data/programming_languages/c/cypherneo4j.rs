//! Hello-world snippet for `ProgrammingLanguageName::Cypherneo4j`.
//!
//! Imported from leachim6/hello-world (`c/CypherNeo4j.cypher` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CypherNeo4j.cypher>
//! - <https://en.wikipedia.org/wiki/Cypher_(query_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Query,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cypherneo4j => "CypherNeo4j",
    versions: [Cypherneo4j => "CypherNeo4j"],
    extensions: [Cypher => "cypher"],
}

const CYPHERNEO4J: &str = concat!(
    r"CREATE (Hello:Word { val: 'Hello' }), (World:Word { val: 'World' }),
(Hello)-[:SPACE]->(World)
RETURN ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cypherneo4j,
        extensions: &[FileExtension::Cypher],
        categories: &[Query],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cypherneo4j,
            syntax: CYPHERNEO4J,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CYPHERNEO4J,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"CREATE (Hello:Word { val: 'Hello' }), (World:Word { val: 'World' }),
(Hello)-[:SPACE]->(World)
RETURN ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
