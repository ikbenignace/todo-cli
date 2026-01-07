use clap::Parser;
use std::process::Command;
use std::str;

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Create a Jira todo ticket in project MS", long_about = None)]
struct Cli {
    #[arg(required = true)]
    summary: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

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

    create_ticket(&cli.summary);
}

fn check_acli_available() -> bool {
    let output = Command::new("acli")
        .arg("--version")
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

fn check_acli_authenticated() -> bool {
    let output = Command::new("acli")
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

fn create_ticket(summary: &str) {
    let output = Command::new("acli")
        .args([
            "jira",
            "workitem",
            "create",
            "--project", "MS",
            "--type", "Task",
            "--summary", summary,
        ])
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let stdout = str::from_utf8(&o.stdout).unwrap_or("");
                print!("{}", stdout);
            } else {
                let stderr = str::from_utf8(&o.stderr).unwrap_or("");
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
