#[derive(Debug, Clone)]
pub struct AsmLine {
    pub label:Option<String>,
    pub instruction:Option<String>,
    pub operands:Option<String>,
    pub comment:Option<String>,
}

impl AsmLine {
    pub fn new_blank() -> Self {
        AsmLine {
            label:None,
            instruction:None,
            operands:None,
            comment:None,
        }
    }
}

pub fn parse_asm(code:String) -> Vec<AsmLine>{
    let lines = get_lines(&code);
    let mut parsed: Vec<AsmLine> = Vec::new();

    for line in lines.iter() {
        parsed.push(parse_line(line.to_string()));
    }

    parsed
}

fn get_lines(code:&str) -> Vec<&str> {
    code.split('\n').map(|x| x.trim()).collect::<Vec<&str>>()
}

fn parse_line(line:String) -> AsmLine {

    let mut blank = AsmLine::new_blank();
    let mut label_end = 0;
    let mut comment_start = line.len();

    // Grab comment
    if let Some(comment_index) = find_comment_start(&line) {
        comment_start = comment_index;
        blank.comment = Some(line[comment_index..].to_owned());
    };

    // grab label
    if let Some(label_index) = line.find(':') {
        let candidate = &line[..=label_index];
        label_end = label_index+1;

        if !candidate.is_empty() && !candidate.chars().any(char::is_whitespace) {
            blank.label = Some(candidate.to_string());
        }
    }

    // grab instruction and operands
    let instr_and_ops = &line[label_end..comment_start].trim();
    let mut symbols = instr_and_ops.split_whitespace().collect::<Vec<&str>>();
    symbols.retain(|x| *x!="");

    if symbols.len() > 0 {
        blank.instruction = Some(symbols[0].to_string());
        if symbols.len() > 1 {
            blank.operands = Some(symbols[1..symbols.len()].join(" "));
        } 
    }

    blank
}

fn find_comment_start(line: &str) -> Option<usize> {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;

    for (i, c) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        if c == '\\' && (single_quoted || double_quoted) {
            escaped = true;
            continue;
        }

        match c {
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            ';' if !single_quoted && !double_quoted => return Some(i),
            _ => {}
        }
    }

    None
}
