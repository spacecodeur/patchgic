use crate::{context, filetypes::FileTypeHandler, utils::file_io::write_file};

use std::error::Error;

use super::regex_handler::RegexHandler;

pub fn process_file(
    file_path: &str,
    file_content: &str,
    regex: &str,
    new_content: &str,
    handler: &dyn FileTypeHandler,
) -> Result<(), Box<dyn Error>> {
    // Étape 1: Trouver tous les matches
    let regex_handler = RegexHandler::new(regex, file_content)?;

    // Étape 2: Détection des contextes
    let common_contexts = context::common::detect(file_content, regex_handler);
    let specific_contexts = Vec::<FileTypeContext>::new(); // À implémenter selon le type de fichier

    // Étape 3: Prétraitement (délégué au handler)
    let processed_content = handler.process(
        file_content,
        full_match.as_str(),
        capture_group,
        new_content,
    )?;

    // Étape 4: Écrire le résultat
    write_file(file_path, &processed_content)?;

    Ok(())
}
