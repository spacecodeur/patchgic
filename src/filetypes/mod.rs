use crate::context::filetype::FileTypeContext;
use crate::processing::regex_handler::RegexHandler;
use std::error::Error;
use std::path::Path;

pub mod common;
pub mod env;

pub trait FileTypeHandler {
    fn get_supported_extensions(&self) -> Vec<&'static str>;

    fn process(
        &self,
        file_content: &str,
        regex_handler: RegexHandler,
        new_content: &str,
    ) -> Result<String, Box<dyn Error>>;

    fn detect_specific_contexts(
        &self,
        file_content: &str,
        regex_handler: RegexHandler,
    ) -> Vec<FileTypeContext>;
}

// Liste des handlers statiques
const HANDLERS: &[&dyn FileTypeHandler] = &[&env::EnvFileHandler];

pub fn get_handler_for_file(
    file_path: &str,
) -> Result<&'static dyn FileTypeHandler, Box<dyn Error>> {
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or("Impossible de déterminer l'extension du fichier")?;

    for handler in HANDLERS {
        if handler.get_supported_extensions().contains(&extension) {
            return Ok(*handler);
        }
    }

    Err(format!("Aucun handler disponible pour les fichiers .{}", extension).into())
}
