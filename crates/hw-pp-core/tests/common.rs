use hw_pp_core::data::common::{
    DEFAULT_HUMAN_LANGUAGE, DEFAULT_HUMAN_LANGUAGE_DIALECT, DEFAULT_PROGRAMMING_LANGUAGE,
    get_default_human_language_hello_world,
};
use hw_pp_core::data::human_languages::{DialectName, HumanLanguageName};
use hw_pp_core::data::programming_languages::ProgrammingLanguageName;

#[test]
fn defaults_are_english_c_and_latin() {
    assert_eq!(DEFAULT_HUMAN_LANGUAGE, HumanLanguageName::English);
    assert_eq!(DEFAULT_HUMAN_LANGUAGE_DIALECT, DialectName::AmericanEnglish);
    assert_eq!(DEFAULT_PROGRAMMING_LANGUAGE, ProgrammingLanguageName::C);
    assert_eq!(get_default_human_language_hello_world(), "Hello, World!");
}
