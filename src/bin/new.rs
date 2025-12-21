use { std::{
        env,
        ffi::{OsString},
        fs,
        io::{self, BufRead},
        process::{Command, ExitCode, Stdio},
        path::{Path, PathBuf},
    },
    colored::Colorize
};

const TEMPLATE_C: &str = include_str!("../../templates/main.c");
const TEMPLATE_RUST: &str = include_str!("../../templates/main.rs");

#[derive(Copy,Clone,Debug)]
pub enum Language {
    C,
    Rust
}

pub struct CommandLineArgs {
    directory: PathBuf,
    language: Language 
}

// TODO: move to Workspace struct
impl CommandLineArgs {
    pub fn main_file(&self) -> &'static str {
        match self.language {
            Language::C => "main.c",
            Language::Rust => "src/main.rs"
        }
    }
}

fn parse_command_line_args() -> Result<CommandLineArgs, String> {
    let raw_args: Vec<OsString> = env::args_os().collect();
    let mut directory: Option<OsString> = None;
    let mut language: Option<OsString> = None;
    let mut positional_args: Vec<OsString> = Vec::new();
    let mut i: usize = 1; // skip first argument
    while i < raw_args.len() {
        match raw_args[i].to_str() {
            Some("-d") | Some("--dir") | Some("--directory") => {
                if directory != None {
                    return Err("Specified directory more than once".to_string());
                }
                directory = Some(raw_args[i+1].clone());
                i += 1;
            },
            Some("-l") | Some("--lang") | Some("--language") => {
                if language != None {
                    return Err("Specified language more than once".to_string());
                }
                language = Some(raw_args[i+1].clone());
                i += 1;
            },
            Some(positional_arg) => {
                positional_args.push(OsString::from(positional_arg));
            }
            None => {
                return Err(format!("Could not parse argument \"{:?}\"", raw_args[i]));
            }
        }
        i += 1;
    }
    if positional_args.len() > 2 {
        return Err("Too many positional arguments, maximum 2: directory and language".to_string());
    }
    for arg in positional_args {
        if directory == None { directory = Some(arg); }
        else if language == None { language = Some(arg); }
        else { return Err("Too many positional arguments".to_string()); }
    }
    let directory = match directory {
        Some(directory) => directory,
        None => return Err("Directory argument missing. \
            Use -d/--dir/--directory or give it as positional argument".to_string())
    };
    let language = match language {
        Some(language) => {
            let parsed_language = language.to_string_lossy().to_ascii_lowercase();
            match parsed_language.as_str() {
                "c" => Language::C,
                "rs" | "rust" => Language::Rust,
                _ => return Err(format!("Invalid language {:?}", language))
            }
        },
        None => Language::Rust
    };
    Ok(CommandLineArgs {
        directory: PathBuf::from(directory),
        language: language
    })
}

fn ensure_dir(dir_path: &Path) -> Result<(), String> {
    if let Err(error) = fs::create_dir_all(&dir_path) {
        let msg = format!("Could not create directory {:?}.\n{}", dir_path, error.to_string());
        return Err(msg);
    } else {
        println!("Ensured directory {:?}", dir_path);
    }
    Ok(())
}

fn write_file(file_path: &Path, content: &str) -> Result<(), String> {
    if let Err(error) = fs::write(file_path, content) {
        let msg = format!("Error writing file {:?}.\n{}", file_path, error.to_string());
        return Err(msg);
    }
    println!("Wrote file {:?}", file_path);
    Ok(())
}

fn create_file(file_path: &Path, content: &str) -> Result<(), String> {
    if file_path.is_file() {
        return Err(format!("File {:?} already exists", file_path));
    }
    write_file(file_path, content)
}

fn read_cf_samples_from_console(data_dir: &Path) -> Result<(),String> {
    println!("Copy and paste the sample test cases from codeforces");
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut content = String::from("");
    let mut case_id = 0;
    let mut is_input = true;
    for line in handle.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                let msg = format!("Error reading line: {}", error.to_string());
                eprintln!("{}", msg.red());
                continue;
            }
        };
        if line == "InputCopy" { }
        else if line == "=" || line == "OutputCopy" || line == "Note" {
            let file_name = match is_input {
                true => {
                    format!("{}.in", case_id)
                },
                false => {
                    let out_file = format!("{}.out", case_id);
                    let out_file = data_dir.join(&out_file);
                    create_file(&out_file, "")?;
                    case_id += 1;
                    format!("{}.ans", case_id-1)
                }
            };
            let file_path = data_dir.join(file_name);
            create_file(&file_path, &content)?;
            content.clear();
            is_input = !is_input;
            if line == "Note" {
                println!("Done reading sample test cases!");
                break;
            }
        } else {
            content += &line;
            content += "\n";
        } 
    }
    Ok(())
}

fn create_workspace(args: &CommandLineArgs) -> Result<(), String> {
    println!("Creating workspace {:?}. Language = {:?}", args.directory, args.language);
    let dir = Path::new(&args.directory);
    if dir.is_file() {
        return Err(format!("The path {:?} already exit as a file", args.directory));
    }
    ensure_dir(&dir)?;
    match args.language {
        Language::C => {
            let main_file = dir.join("main.c");
            create_file(&main_file, TEMPLATE_C)?;
        },
        Language::Rust => {
            println!("Running cargo init for {:?}..", dir);
            let mut command = Command::new("cargo");
            let command = command
                .arg("init")
                .arg(&dir)
                .stderr(Stdio::inherit())
                .stdout(Stdio::inherit());
            let mut spanwed_command = match command.spawn() {
                Ok(spanwed) => spanwed,
                Err(error) => {
                    let msg = format!("Error running `cargo init`: {:?}", error.to_string());
                    return Err(msg);
                }
            };
            match spanwed_command.wait() {
                Ok(code) => {
                    if !code.success() {
                        let msg = format!("cargo init failed with status code {}", code);
                        return Err(msg);
                    }
                },
                Err(error) => {
                    let msg = format!("Error in `cargo init`: {:?}", error.to_string());
                    return Err(msg);
                }
            };
            let main_file = dir.join("src").join("main.rs");
            write_file(&main_file, TEMPLATE_RUST)?;
        }
    }
    read_cf_samples_from_console(&dir)?;
    Ok(())
}

fn start_nvim(args: &CommandLineArgs) -> Result<(), String> {
    if let Err(error) = env::set_current_dir(&args.directory) {
        let msg = format!(
            "Could not change directory to {:?}: {}",
            args.directory,
            error.to_string());
        return Err(msg);
    }
    let mut nvim_command = Command::new("nvim");
    let nvim_command = nvim_command
        .arg(args.main_file())
        .arg("0.in")
        .arg("0.ans")
        .arg("0.out");
    let mut nvim_command = match nvim_command.spawn() {
        Ok(cmd) => cmd,
        Err(error) => return Err(error.to_string())
    };
    match nvim_command.wait() {
        Ok(code) => {
            if !code.success() {
                let msg = format!("nvim failed with status code {}", code);
                return Err(msg);
            }
        },
        Err(error) => return Err(error.to_string())
    }
    Ok(())
}

fn main() -> ExitCode {
    let args = match parse_command_line_args() {
        Ok(args) => args,
        Err(error) => {
            let msg = format!("Error parsing command line arguments: {}", error);
            eprintln!("{}", msg.bold().red());
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = create_workspace(&args) {
        let msg = format!("Error creating workspace: {}", error);
        eprintln!("{}", msg.bold().red());
        return ExitCode::FAILURE;
    }

    if let Err(error) = start_nvim(&args) {
        let msg = format!("Error opening workspace with nvim: {}", error);
        eprintln!("{}", msg.bold().red());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
