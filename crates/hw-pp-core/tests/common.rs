use hw_pp_core::data::common::{
    DEFAULT_DIALECT, DEFAULT_HUMAN_LANGUAGE, DEFAULT_PROGRAMMING_LANGUAGE, default_greeting,
};
use hw_pp_core::data::human_languages::{DialectName, HumanLanguageName};
use hw_pp_core::data::programming_languages::ProgrammingLanguageName;

#[test]
fn defaults_are_english_c_and_latin() {
    assert_eq!(DEFAULT_HUMAN_LANGUAGE, HumanLanguageName::English);
    assert_eq!(DEFAULT_DIALECT, DialectName::AmericanEnglish);
    assert_eq!(DEFAULT_PROGRAMMING_LANGUAGE, ProgrammingLanguageName::C);
    assert_eq!(default_greeting(), "Hello, World!");
}
