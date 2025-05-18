pub mod compose_yml;
pub mod env;

#[derive(Debug, PartialEq)]
pub enum FileTypeContext {
    Env(env::EnvContext),
    // ComposeYml(compose_yml::ComposeYmlContext),
    // ... autres types de fichiers
}
