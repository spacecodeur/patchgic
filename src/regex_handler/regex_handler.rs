use regex::RegexBuilder;

#[derive(Clone)]
pub struct RegexHandler {
    pub content_matched: ContentMatched,
    pub captured_group: Option<CapturedGroup>,
    pub contexts: 
}

#[derive(Clone)]
struct ContentMatched {
    pub content: String,
    pub num_line_start: usize,
    pub num_col_start: usize,
    pub num_line_end: usize,
    pub num_col_end: usize,
}

#[derive(Clone)]
struct CapturedGroup {
    pub content: String,
    pub num_line_start: usize,
    pub num_col_start: usize,
    pub num_line_end: usize,
    pub num_col_end: usize,
}

#[derive(Clone)]
struct Contextes {
    pub common: Vec<ContextCommon>,
    pub filetype: Vec<ContextFileType>
}

impl RegexHandler {
    pub fn new(regex: &str, file_content: &str) -> Result<RegexHandler, String> {
        let re = RegexBuilder::new(&regex)
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .expect("Invalid regex");

        // Vérification qu'il y a exactement un groupe de capture
        if re.captures_len() != 2 {
            return Err("La regex doit contenir exactement un groupe de capture".into());
        }

        if let Some(caps) = re.captures(&file_content) {
            let mat = caps.get(0).unwrap();

            let (cm_start_line, cm_start_col) = get_line_col(&file_content, mat.start());
            let (cm_end_line, cm_end_col) = get_line_col(&file_content, mat.end());

            let mut result = RegexHandler {
                content_matched: ContentMatched {
                    content: mat.as_str().to_string(),
                    num_line_start: cm_start_line,
                    num_col_start: cm_start_col,
                    num_line_end: cm_end_line,
                    num_col_end: cm_end_col,
                },
                captured_group: None,
            };

            let group = caps.get(1);

            if group.is_some() {
                let group = group.unwrap();

                let (cg_start_line, cg_start_col) = get_line_col(&file_content, group.start());
                let (cg_end_line, cg_end_col) = get_line_col(&file_content, group.end());

                result.captured_group = Some(CapturedGroup {
                    content: group.as_str().to_string(),
                    num_line_start: cg_start_line,
                    num_col_start: cg_start_col,
                    num_line_end: cg_end_line,
                    num_col_end: cg_end_col,
                });
            }

            Ok(result)
        } else {
            return Err("No match found".into());
        }
    }
}

/// Renvoie le numéro de ligne et de colonne (commençant à 1) pour un index donné.
fn get_line_col(text: &str, byte_idx: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    let mut count = 0;

    for l in text.lines() {
        let len = l.len() + 1; // +1 for '\n'
        if count + len > byte_idx {
            col = byte_idx - count + 1;
            break;
        }
        count += len;
        line += 1;
    }

    (line, col)
}
