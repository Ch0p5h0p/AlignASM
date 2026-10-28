use super::parser::*;

pub struct FormatGuide {
    label_size:usize,
    instruction_size:usize,
    operands_size:usize,
}

pub fn format_lines(lines:Vec<AsmLine>) -> String {
    let format_guide = make_guide(&lines);
    let collapsed = collapse_lines(lines);
    collapsed.into_iter()
        .map(|line| format_line(line, &format_guide))
        .collect()
}

fn format_line(line:AsmLine, guide:&FormatGuide) -> String { 
    let label = line.label.as_deref().unwrap_or("");
    let instruction = line.instruction.as_deref().unwrap_or("");
    let operands = line.operands.as_deref().unwrap_or("");
    let comment = line.comment.as_deref().unwrap_or("");
    
    let label_size = guide.label_size;
    let instr_size = guide.instruction_size;
    let opers_size = guide.operands_size;

    format!("{label:<label_size$}    {instruction:<instr_size$}    {operands:<opers_size$}    {comment}\n")
}

fn make_guide(lines:&Vec<AsmLine>) -> FormatGuide {
    let mut label_size: usize = 0;
    let mut instruction_size: usize = 0;
    let mut operands_size: usize = 0;

    for line in lines.iter() {
        if let Some(label) = &line.label {
            if label.len() > label_size {
                label_size = label.len();
            }
        }

        if let Some(instruction) = &line.instruction {
            if instruction.len() > instruction_size {
                instruction_size = instruction.len();
            }
        }

        if let Some(operands) = &line.operands {
            if operands.len() > operands_size {
                operands_size = operands.len();
            }
        }
    }

    FormatGuide {
        label_size,
        instruction_size,
        operands_size,
    }
}

fn collapse_lines(lines:Vec<AsmLine>) -> Vec<AsmLine> {
    let mut collapsed: Vec<AsmLine> = Vec::new();
    let mut lines_iter = lines.iter();
    let mut stored = AsmLine::new_blank();

    while let Some(line) = lines_iter.next() {
        let mut current = line.clone();

        if line.label != None 
            && line.comment == None
            && line.instruction == None
        {
            stored.label = line.label.clone();
            continue;
        }
        if current.label == None && stored.label != None {
            current.label = stored.label;
            stored.label = None;
        }

        collapsed.push(current);
    };

    collapsed
}

