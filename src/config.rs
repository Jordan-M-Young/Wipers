use serde::Deserialize;
use crate::file::load_file_string;

#[derive(Deserialize,Debug,PartialEq)]
pub struct OpenAiConfig {
    pub key: String,
    pub org_id: String
}

#[derive(Deserialize,Debug,PartialEq)]
pub struct LockConfig {
    pub lockfiles: Vec<String>
}



#[derive(Deserialize,Debug,PartialEq)]
pub struct TomlConfig{
    pub openai: OpenAiConfig,
    pub lock: LockConfig
}



pub fn load_toml(toml_filename: &str) -> TomlConfig {
    let toml_string = load_file_string(toml_filename).unwrap();
    let toml_config: TomlConfig = toml::from_str(&toml_string).unwrap();
    toml_config



}


#[cfg(test)]
mod tests {
    use super::{load_toml, TomlConfig,OpenAiConfig, LockConfig};


    #[test]

    fn test_load_good_toml() {
        let toml_filename = "./test-assets/good.toml";
        let actual_toml_conifg = load_toml(toml_filename);

        let openai_config = OpenAiConfig{
            key: "".to_string(),
            org_id: "".to_string()
        };

        let lock_config = LockConfig{
            lockfiles: vec![]
        };

        let expected_toml_config = TomlConfig{
            openai: openai_config,
            lock: lock_config

        };


        assert_eq!(actual_toml_conifg,expected_toml_config)

    
    }

    #[test]
    #[should_panic(expected="called `Result::unwrap()` on an `Err` value: Error { inner: Error { inner: TomlError { message: \"missing field `key`\", original: Some(\"[openai]\\norg_id = \\\"\\\"\\n\\n[lock]\\nlockfiles = []\"), keys: [\"openai\"], span: Some(0..20) } } }")]
    fn test_load_missing_openai_key_toml() {
        let toml_filename = "./test-assets/bad_openai_key.toml";
        let _actual_toml_conifg = load_toml(toml_filename);

    }
    #[test]
    #[should_panic(expected="called `Result::unwrap()` on an `Err` value: Error { inner: Error { inner: TomlError { message: \"missing field `org_id`\", original: Some(\"[openai]\\nkey = \\\"\\\"\\n\\n[lock]\\nlockfiles = []\"), keys: [\"openai\"], span: Some(0..17) } } }")]
    fn test_load_missing_openai_orgid_toml(){
        let toml_filename = "./test-assets/bad_openai_org_id.toml";
        let _actual_toml_conifg = load_toml(toml_filename);

    }

    #[test]
    #[should_panic(expected="called `Result::unwrap()` on an `Err` value: Error { inner: Error { inner: TomlError { message: \"missing field `lockfiles`\", original: Some(\"[openai]\\nkey = \\\"\\\"\\norg_id = \\\"\\\"\\n\\n[lock]\\n\"), keys: [\"lock\"], span: Some(31..37) } } }")]
    fn test_load_missing_lock_toml() {
        let toml_filename = "./test-assets/bad_lock.toml";
        let _actual_toml_conifg = load_toml(toml_filename);

    }


}

