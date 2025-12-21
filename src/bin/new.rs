use {
    std::{
        env,
        fs,
        fmt,
        io::{self, BufRead},
        process::{Command, ExitCode, Stdio},
        path::{Path, PathBuf},
    },
    colored::Colorize,
    clap::{Parser, ValueEnum}
};

const TEMPLATE_C: &str = include_str!("../../templates/main.c");
const TEMPLATE_RUST: &str = include_str!("../../templates/main.rs");

#[derive(Copy, Clone, Debug, Parser, ValueEnum)]
pub enum Language {
    C,
    Rust
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

/// Create a new workspace for problem solving.
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct CommandLineArgs {
    /// Directory in which to create the workspace.
    #[arg(value_name="directory")]
    directory: PathBuf,
    /// Programming language.
    #[arg(value_name="language", default_value="rust")]
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

fn ensure_dir(dir_path: &Path) -> Result<(), String> {
    if let Err(error) = fs::create_dir_all(&dir_path) {
        let msg = format!("Could not create directory {:?}.\n{}", dir_path, error.to_string());
        return Err(msg);
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
    let msg = "Copy and paste the problem from Codeforces and then press enter \
             (Ctrl-A + Ctrl-C in the browser, then Ctrl-V + <Enter> here)";
    println!("{}", String::from(msg).cyan());
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut content = String::from("");
    let mut case_id = 0;
    let mut is_input = true;
    let mut cases_reached = false;
    let mut cases_finished = false;
    for line in handle.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                let msg = format!("Error reading line: {}", error.to_string());
                eprintln!("{}", msg.red());
                continue;
            }
        };
        if cases_finished {
            if line.contains("ITMO University") { break; }
            continue;
        }
        else if line == "InputCopy" { cases_reached = true; }
        else if !cases_reached { continue; }
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
            if line == "Note" { cases_finished = true; }
        } else {
            content += &line;
            content += "\n";
        }
    }
    println!("Done reading sample test cases!");
    Ok(())
}

fn create_workspace(args: &CommandLineArgs) -> Result<(), String> {
    let msg = format!("Creating workspace {:?}. Language = {:?}", args.directory, args.language);
    println!("{}", msg.cyan());
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
    let args = CommandLineArgs::parse();

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
