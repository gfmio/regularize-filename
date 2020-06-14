use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

enum FormattingMode {
    SnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
}

/// Regularize the names of your files according to your favourite naming convention
#[derive(StructOpt, Debug)]
#[structopt(name = "regularize-filename")]
struct Opts {
    /// Use kebab-case
    #[structopt(short, long)]
    kebab_case: bool,

    /// Use snake_case
    #[structopt(short, long)]
    snake_case: bool,

    /// Use camelCase
    #[structopt(short, long)]
    camel_case: bool,

    /// Use PascalCase
    #[structopt(short, long)]
    pascal_case: bool,
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn parse_filename(input: String) -> Vec<String> {
    let mut parts = Vec::<String>::new();
    let mut current = String::new();
    let mut previous_char_was_uppercase = false;

    for current_char in input.chars() {
        let current_char_is_alphabetic = current_char.is_alphabetic();
        let current_char_is_numeric = current_char.is_numeric();
        let current_char_is_alphanumeric = current_char_is_alphabetic || current_char_is_numeric;
        let current_char_is_uppercase = current_char.is_uppercase();
        let current_char_is_lowercase = current_char.is_lowercase();
        let same_case_as_previous = (current_char_is_uppercase && previous_char_was_uppercase)
            || (current_char_is_lowercase && !previous_char_was_uppercase);

        if current_char_is_alphanumeric {
            if current_char_is_alphabetic {
                previous_char_was_uppercase = current_char_is_uppercase;
            }
            if current.len() < 2 || same_case_as_previous {
                current.push(current_char);
                continue;
            }
        }

        if current.len() > 0 {
            parts.push(current);
            current = String::new();
            if current_char_is_alphanumeric {
                current.push(current_char);
            }
        }
    }
    if current.len() > 0 {
        parts.push(current);
    }
    return parts;
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn snake_case(parts: Vec<String>) -> String {
    return parts
        .iter()
        .map(|part| part.to_lowercase())
        .collect::<Vec<String>>()
        .join("_");
}

fn kebab_case(parts: Vec<String>) -> String {
    return parts
        .iter()
        .map(|part| part.to_lowercase())
        .collect::<Vec<String>>()
        .join("-");
}

fn camel_case(parts: Vec<String>) -> String {
    let mut s = String::new();
    for part in parts {
        if s.len() == 0 {
            s = s + &part.to_lowercase();
        } else {
            let (car, cdr) = car_cdr(&part);
            s = s + &car.to_uppercase() + &cdr.to_lowercase();
        }
    }
    return s;
}

fn pascal_case(parts: Vec<String>) -> String {
    let mut s = String::new();
    for part in parts {
        let (car, cdr) = car_cdr(&part);
        s = s + &car.to_uppercase() + &cdr.to_lowercase();
    }
    return s;
}

fn convert_filename(filename: &str, mode: &FormattingMode) -> String {
    let parts = parse_filename(String::from(filename));
    match mode {
        FormattingMode::CamelCase => return camel_case(parts),
        FormattingMode::KebabCase => return kebab_case(parts),
        FormattingMode::PascalCase => return pascal_case(parts),
        FormattingMode::SnakeCase => return snake_case(parts),
    };
}

const DOT: &str = ".";

fn main() -> std::io::Result<()> {
    let opts = Opts::from_args();

    // Determine mode
    let mode;
    if opts.kebab_case {
        mode = FormattingMode::KebabCase;
    } else if opts.camel_case {
        mode = FormattingMode::CamelCase;
    } else if opts.pascal_case {
        mode = FormattingMode::PascalCase;
    } else if opts.snake_case {
        mode = FormattingMode::SnakeCase;
    } else {
        // Default
        mode = FormattingMode::KebabCase;
    }

    // Go through all the files and rename them
    for file in opts.files {
        let old_path = file.as_path();
        let parent = old_path.parent().unwrap().to_str().unwrap();
        let stem = old_path.file_stem();
        let extension = old_path.extension();

        let new_extension;
        if extension.is_some() {
            new_extension = ".".to_owned() + &extension.unwrap().to_str().unwrap().to_lowercase();
        } else {
            new_extension = "".to_owned();
        }

        let formatted_filename;
        if stem.is_some() {
            formatted_filename = stem
                .unwrap()
                .to_str()
                .unwrap()
                .split(DOT)
                .collect::<Vec<&str>>()
                .iter()
                .map(|&part| convert_filename(part, &mode))
                .collect::<Vec<String>>()
                .join(DOT);
        } else {
            formatted_filename = "".to_owned();
        }

        let new_path: PathBuf = vec![parent, &(formatted_filename + &new_extension)]
            .iter()
            .collect();

        println!(
            "mv {} {}",
            old_path.to_str().unwrap(),
            new_path.to_str().unwrap()
        );
        fs::rename(old_path, new_path)?;
    }

    Ok(())
}
