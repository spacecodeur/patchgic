use crate::{context::filetype::FileTypeContext, processing::regex_handler::RegexHandler};

use super::FileTypeHandler;
use std::error::Error;

pub struct EnvFileHandler;

impl FileTypeHandler for EnvFileHandler {
    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec!["env"]
    }

    fn process(
        &self,
        file_content: &str,
        regex_handler: RegexHandler,
        new_content: &str,
    ) -> Result<String, Box<dyn Error>> {
        // Logique spécifique pour les fichiers .env
        let updated_match = regex_handler
            .content_matched
            .content
            .replace(&regex_handler.captured_group.unwrap().content, new_content);
        let updated_content =
            file_content.replace(&regex_handler.content_matched.content, &updated_match);

        Ok(updated_content)
    }

    fn detect_specific_contexts(
        &self,
        file_content: &str,
        regex_handler: RegexHandler,
    ) -> Vec<FileTypeContext> {
        use crate::context::filetype::env::detect_env_specific_contexts;
        use FileTypeContext::Env;

        detect_env_specific_contexts(file_content, regex_handler)
            .into_iter()
            .map(Env)
            .collect()
    }
}
