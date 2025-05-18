pub mod common;
pub mod filetype;

use common::CommonContext;
use filetype::FileTypeContext;

use crate::{filetypes::FileTypeHandler, processing::regex_handler::RegexHandler};

#[derive(Debug, PartialEq)]
pub enum Context {
    Common(CommonContext),
    FileTypeSpecific(FileTypeContext),
}

pub fn detect_all_contexts(
    regex_handler: RegexHandler,
    file_content: &str,
    file_handler: &dyn FileTypeHandler,
) -> Vec<Context> {
    let mut contexts = Vec::new();

    // Ajout des contextes communs
    for common_ctx in common::detect(file_content, regex_handler.clone()) {
        contexts.push(Context::Common(common_ctx));
    }

    // Ajout des contextes spécifiques
    for specific_ctx in file_handler.detect_specific_contexts(file_content, regex_handler) {
        contexts.push(Context::FileTypeSpecific(specific_ctx));
    }

    contexts
}
