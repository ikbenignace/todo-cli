use clap::{CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Shell};
use regex::Regex;
use serde::Deserialize;
use std::process::Command as ProcessCommand;
use std::str;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Create a Jira todo ticket", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[arg(
        value_name = "PROJECT",
        help = "Jira project key (e.g., MS, AA)",
        long_help = "Jira project key. Leave empty to use default (MS).",
        value_hint = ValueHint::Other,
        num_args = 0..=1
    )]
    project: Option<String>,

    #[arg(
        value_name = "SUMMARY",
        help = "Ticket summary/description",
        long_help = "The summary or description of the Jira ticket to create.",
        num_args = 0..=1
    )]
    summary: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate shell completion scripts")]
    Completion {
        #[arg(short, long, help = "Shell type (bash, zsh, fish, elvish, powershell)")]
        shell: Shell,
    },
}

#[derive(Deserialize)]
struct Project {
    key: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Completion { shell }) = cli.command {
        print_completions(shell);
        return;
    }

    if !check_acli_available() {
        eprintln!("Error: acli is not installed or not in PATH");
        show_installation_instructions();
        std::process::exit(1);
    }

    if !check_acli_authenticated() {
        eprintln!("Error: acli is not authenticated");
        eprintln!("Please run: acli jira auth login --web");
        std::process::exit(1);
    }

    let project = cli.project.unwrap_or_else(|| "MS".to_string());
    let summary = match cli.summary {
        Some(s) => s,
        None => {
            eprintln!("Error: SUMMARY is required");
            eprintln!("Usage: todo [PROJECT] SUMMARY");
            eprintln!("Examples:");
            eprintln!("  todo 'fix bug in login'");
            eprintln!("  todo MS 'implement new feature'");
            eprintln!("  todo AA 'create epic'");
            std::process::exit(1);
        }
    };

    create_ticket(&project, &summary);
}

fn print_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut std::io::stdout());
}

fn check_acli_available() -> bool {
    let output = ProcessCommand::new("acli")
        .arg("--version")
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

fn check_acli_authenticated() -> bool {
    let output = ProcessCommand::new("acli")
        .args(["jira", "auth", "status"])
        .output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let stderr = String::from_utf8_lossy(&o.stderr);
            let output = format!("{}{}", stdout, stderr);
            output.contains("Authenticated")
        }
        Err(_) => false,
    }
}

fn get_projects() -> Vec<String> {
    let output = ProcessCommand::new("acli")
        .args(["jira", "project", "list", "--json", "--limit", "100"])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            match serde_json::from_str::<Vec<Project>>(&stdout) {
                Ok(projects) => projects.iter().map(|p| p.key.clone()).collect(),
                Err(_) => vec![],
            }
        }
        _ => vec![],
    }
}

fn create_ticket(project: &str, summary: &str) {
    let output = ProcessCommand::new("acli")
        .args([
            "jira",
            "workitem",
            "create",
            "--project", project,
            "--type", "Story",
            "--summary", summary,
            "--assignee", "@me",
        ])
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let stdout = String::from_utf8_lossy(&o.stdout);
                print!("{}", stdout);

                if let Some(ticket_key) = parse_ticket_key(&stdout, project) {
                    transition_ticket(&ticket_key);
                }
            } else {
                let stderr = String::from_utf8_lossy(&o.stderr);
                eprintln!("Error creating ticket: {}", stderr);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error running acli: {}", e);
            std::process::exit(1);
        }
    }
}

fn parse_ticket_key(output: &str, project: &str) -> Option<String> {
    let re = Regex::new(&format!(r"{}-\d+", project)).unwrap();
    re.find(output).map(|m| m.as_str().to_string())
}

fn transition_ticket(ticket_key: &str) {
    let output = ProcessCommand::new("acli")
        .args([
            "jira",
            "workitem",
            "transition",
            "--key", ticket_key,
            "--status", "Selected for Development",
            "--yes",
        ])
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let stdout = String::from_utf8_lossy(&o.stdout);
                if !stdout.is_empty() {
                    println!("{}", stdout.trim());
                }
            } else {
                let stderr = String::from_utf8_lossy(&o.stderr);
                eprintln!("Warning: Could not transition ticket status: {}", stderr);
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not transition ticket status: {}", e);
        }
    }
}

fn show_installation_instructions() {
    let os = std::env::consts::OS;

    match os {
        "macos" => {
            println!("\nInstall acli on macOS:");
            println!("  Option 1 (Homebrew):");
            println!("    brew tap atlassian/homebrew-acli");
            println!("    brew install acli");
            println!("\n  Option 2 (curl):");
            println!("    curl -LO \"https://acli.atlassian.com/darwin/latest/acli_darwin_$(uname -m | sed 's/arm64/arm64/;s/x86_64/amd64/')/acli\"");
            println!("    chmod +x ./acli");
            println!("    sudo mv ./acli /usr/local/bin/acli");
        }
        "linux" => {
            println!("\nInstall acli on Linux:");
            println!("  Option 1 (Debian/Ubuntu):");
            println!("    sudo apt-get install -y wget gnupg2");
            println!("    sudo mkdir -p -m 755 /etc/apt/keyrings");
            println!("    wget -nv -O- https://acli.atlassian.com/gpg/public-key.asc | sudo gpg --dearmor -o /etc/apt/keyrings/acli-archive-keyring.gpg");
            println!("    sudo chmod go+r /etc/apt/keyrings/acli-archive-keyring.gpg");
            println!("    echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/acli-archive-keyring.gpg] https://acli.atlassian.com/linux/deb stable main\" | sudo tee /etc/apt/sources.list.d/acli.list > /dev/null");
            println!("    sudo apt update");
            println!("    sudo apt install -y acli");
            println!("\n  Option 2 (Red Hat/CentOS):");
            println!("    sudo yum install -y yum-utils");
            println!("    sudo yum-config-manager --add-repo https://acli.atlassian.com/linux/rpm/acli.repo");
            println!("    sudo yum install -y acli");
            println!("\n  Option 3 (curl):");
            println!("    curl -LO \"https://acli.atlassian.com/linux/latest/acli_linux_$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')/acli\"");
            println!("    chmod +x ./acli");
            println!("    sudo install -o root -g root -m 0755 acli /usr/local/bin/acli");
        }
        "windows" => {
            println!("\nInstall acli on Windows:");
            println!("  In PowerShell:");
            println!("    Invoke-WebRequest -Uri https://acli.atlassian.com/windows/latest/acli_windows_amd64/acli.exe -OutFile acli.exe");
            println!("  Move acli.exe to a directory in your PATH");
        }
        _ => {
            println!("\nVisit https://developer.atlassian.com/cloud/acli/guides/install-acli/ for installation instructions");
        }
    }
}
