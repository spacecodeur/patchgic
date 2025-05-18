use std::error::Error;

mod args;
pub mod context;
pub mod filetypes;
pub mod processing;
pub mod utils;

use args::{Config, parse_args};

use crate::{
    filetypes::get_handler_for_file, processing::pipeline::process_file as pipeline_process,
    utils::file_io::read_file,
};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = parse_args();

    println!("Parsed configuration: {:?}", config);

    process_file(
        &config.filepath,
        &config.replacement_type,
        &config.regex,
        &config.content,
    )?;

    println!("Opération terminée avec succès !");
    Ok(())
}

pub fn process_file(
    file_path: &str,
    replacement_type: &str,
    regex: &str,
    new_content: &str,
) -> Result<(), Box<dyn Error>> {
    // Validation des inputs
    if file_path.trim().is_empty() {
        return Err("Le chemin du fichier ne peut pas être vide".into());
    }

    if regex.trim().is_empty() {
        return Err("La regex ne peut pas être vide".into());
    }

    // Lecture du fichier
    let file_content = read_file(file_path)?;

    // Récupération du handler approprié pour le type de fichier
    let handler = get_handler_for_file(file_path)?;

    // Appel de la pipeline de traitement principale
    pipeline_process(file_path, &file_content, &regex, new_content, handler)?;

    Ok(())
}
