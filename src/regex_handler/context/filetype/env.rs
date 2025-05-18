use crate::processing::regex_handler::RegexHandler;

use super::FileTypeContext;

#[derive(Debug, PartialEq, Clone)]
pub enum EnvContext {
    IsKeyValuePair,
    HasComment,
    IsQuotedValue,
    // ... autres contextes spécifiques aux .env
}

impl From<EnvContext> for FileTypeContext {
    fn from(ctx: EnvContext) -> Self {
        FileTypeContext::Env(ctx)
    }
}

pub fn detect_env_specific_contexts(content: &str, regex_handler: RegexHandler) -> Vec<EnvContext> {
    let mut contexts = Vec::new();
    let matched_text = regex_handler.content_matched.content;

    // Détection si c'est une paire clé=valeur
    if matched_text.contains('=') {
        contexts.push(EnvContext::IsKeyValuePair);
    }

    // Détection si la valeur est entre guillemets
    if matched_text
        .split('=')
        .nth(1)
        .map_or(false, |v| v.contains('"'))
    {
        contexts.push(EnvContext::IsQuotedValue);
    }

    // Détection s'il y a un commentaire
    if matched_text.contains('#') {
        contexts.push(EnvContext::HasComment);
    }

    contexts
}
