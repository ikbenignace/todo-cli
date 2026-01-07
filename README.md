# Todo CLI - Simple Jira Ticket Creator

A simple command-line tool to create Jira tickets in project MS with just one command.

## Quick Start

### Option 1: Quick Install (Recommended)

#### macOS or Linux
```bash
curl -fsSL https://raw.githubusercontent.com/ikbenignace/todo-cli/main/scripts/install.sh | sh
```

#### Windows
```powershell
powershell -ExecutionPolicy Bypass -c "irm https://raw.githubusercontent.com/ikbenignace/todo-cli/main/scripts/install.ps1 | iex"
```

### Option 2: Manual Download

Go to the [Releases page](https://github.com/ikbenignace/todo-cli/releases) and download the file for your system:

**macOS:**
- **MacBook with Apple Silicon (M1/M2/M3):** Download `todo-macos-arm64`
- **MacBook with Intel processor:** Download `todo-macos-x86_64`

**Linux:**
- **ARM64 (e.g., Raspberry Pi):** Download `todo-linux-arm64`
- **Intel/AMD (64-bit):** Download `todo-linux-x86_64`

**Windows:**
- Download `todo-windows-x86_64.exe`

## Installation Guide (Step-by-Step)

### For macOS Users

#### Step 1: Download the File
- Visit [Releases page](https://github.com/ikbenignace/todo-cli/releases)
- Download the appropriate file for your Mac (see "Option 2" above)
- Save it to your Downloads folder

#### Step 2: Make It Executable
Open Terminal and run:
```bash
cd ~/Downloads
chmod +x todo-macos-*
```

#### Step 3: Move It to Your Path
```bash
sudo mv todo-macos-* /usr/local/bin/todo
```

#### Step 4: Verify Installation
```bash
todo --version
```

If you see the help message, you're ready to go!

### For Linux Users

#### Step 1: Download the File
- Visit [Releases page](https://github.com/ikbenignace/todo-cli/releases)
- Download the appropriate file for your system
- Save it to your Downloads folder

#### Step 2: Make It Executable
Open Terminal and run:
```bash
cd ~/Downloads
chmod +x todo-linux-*
```

#### Step 3: Move It to Your Path
```bash
sudo mv todo-linux-* /usr/local/bin/todo
```

#### Step 4: Verify Installation
```bash
todo --version
```

### For Windows Users

#### Step 1: Download the File
- Visit [Releases page](https://github.com/ikbenignace/todo-cli/releases)
- Download `todo-windows-x86_64.exe`
- Save it to a folder, e.g., `C:\Users\YourUsername\bin\todo.exe`

#### Step 2: Add to PATH (One-time setup)
1. Press `Win + R`, type `sysdm.cpl`, and press Enter
2. Click the "Advanced" tab
3. Click "Environment Variables"
4. Under "User variables", find "Path" and click "Edit"
5. Click "New" and add the folder path (e.g., `C:\Users\YourUsername\bin`)
6. Click "OK" on all windows
7. **Important:** Close and reopen Command Prompt or PowerShell

#### Step 3: Verify Installation
Open Command Prompt or PowerShell:
```cmd
todo --help
```

If you see the help message, you're ready to go!

## Prerequisites

Before using Todo CLI, you need to have **acli** (Atlassian CLI) installed and authenticated.

### Install acli (Required)

**macOS:**
```bash
brew tap atlassian/homebrew-acli
brew install acli
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt-get install -y wget gnupg2
sudo mkdir -p -m 755 /etc/apt/keyrings
wget -nv -O- https://acli.atlassian.com/gpg/public-key.asc | sudo gpg --dearmor -o /etc/apt/keyrings/acli-archive-keyring.gpg
sudo chmod go+r /etc/apt/keyrings/acli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/acli-archive-keyring.gpg] https://acli.atlassian.com/linux/deb stable main" | sudo tee /etc/apt/sources.list.d/acli.list > /dev/null
sudo apt update
sudo apt install -y acli
```

**Windows:**
```powershell
Invoke-WebRequest -Uri https://acli.atlassian.com/windows/latest/acli_windows_amd64/acli.exe -OutFile acli.exe
```
Then add `acli.exe` to your PATH (same process as for `todo.exe`).

### Authenticate with Jira
Run this command (opens a browser):
```bash
acli jira auth login --web
```

Follow the instructions in your browser to complete authentication.

## Usage

Create a todo ticket in project MS:

```bash
todo "fix bug in login system"
todo "implement new feature for dashboard"
todo "update documentation for API"
```

### What happens when you run this command?

1. The tool checks if acli is installed
2. It verifies you're authenticated with Jira
3. It creates a Story-type ticket in project MS
4. It shows you the ticket number (e.g., "MS-123")

### Example Output

```
$ todo "implement user authentication feature"
✓ Work item MS-123 created: https://yourcompany.atlassian.net/browse/MS-123
```

## Troubleshooting

### "acli is not installed or not in PATH"
You need to install acli first. See the "Install acli" section above.

### "acli is not authenticated"
Run this command to authenticate:
```bash
acli jira auth login --web
```

### "todo: command not found" (macOS/Linux)
The binary isn't in your PATH. Try:
- Check if you moved it to `/usr/local/bin/` or `~/.local/bin/`
- Add this to your shell config (`~/.bashrc`, `~/.zshrc`):
  ```bash
  export PATH="$PATH:/usr/local/bin"
  ```

### "'todo' is not recognized" (Windows)
You need to add the folder containing `todo.exe` to your PATH. See the Windows installation guide above.

### Command not found after quick install on macOS/Linux
Add this to your shell config (`~/.bashrc`, `~/.zshrc`):
```bash
export PATH="$PATH:$HOME/.local/bin"
```

Then restart your terminal or run:
```bash
source ~/.bashrc  # or source ~/.zshrc
```

## Building from Source (Advanced)

If you want to build it yourself, you need Rust installed:

```bash
git clone https://github.com/ikbenignace/todo-cli.git
cd todo-cli
cargo build --release
cp target/release/todo /usr/local/bin/todo  # macOS/Linux
```

## Support

- GitHub Issues: [https://github.com/ikbenignace/todo-cli/issues](https://github.com/ikbenignace/todo-cli/issues)
- Atlassian CLI Documentation: [https://developer.atlassian.com/cloud/acli/](https://developer.atlassian.com/cloud/acli/)

## License

MIT License - See LICENSE file for details
