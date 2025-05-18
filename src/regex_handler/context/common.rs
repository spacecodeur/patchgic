use crate::processing::regex_handler::RegexHandler;

#[derive(Debug, PartialEq)]
pub enum CommonContext {
    EmptyCaptureGroup,
    MatchOnLastLine,
    BetweenDelimiters(char, char),
}

pub fn detect(file_content: &str, regex_handler: RegexHandler) -> Vec<CommonContext> {
    let mut contexts = Vec::new();

    if regex_handler.captured_group.is_none() {
        contexts.push(CommonContext::EmptyCaptureGroup);
    }

    if regex_handler.content_matched.num_line_end == file_content.lines().count() {
        contexts.push(CommonContext::MatchOnLastLine);
    }

    contexts
}
