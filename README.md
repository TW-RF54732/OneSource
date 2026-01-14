# OneSource

OneSource is a high-efficiency project code aggregator designed specifically for **Vibe Coding**. It consolidates source code from any project into a single structured text file, making it easy to provide full project context to Large Language Models (LLMs) in one step.

---

## Installation

### For Windows Users (CLI Executable)

If you are using the pre-compiled version:

1. **Download** `OneSource.exe` from the [Releases] section.
2. **Move** the file to a dedicated folder (e.g., `C:\Tools\`).
3. **Add to PATH**:
* Open the Start Search, type "env", and select "Edit the system environment variables".
* Click "Environment Variables", find "Path" under "User variables", and click "Edit".
* Click "New" and add the path to the folder where you saved `OneSource.exe`.


4. **Usage**: Open a new Terminal/Command Prompt and run `OneSource` directly from any project directory.

### For Python Users

OneSource requires Python 3.8 or higher.

```bash
# Install dependencies
pip install pathspec pyperclip tiktoken

```

---

## Usage Guide

### 1. Standard Aggregation

Run in the project root to generate `allCode.txt`:

```bash
OneSource
# Or if using the script: python app.py

```

### 2. Flexible Filtering (Gitignore Style)

OneSource now supports advanced inclusion and exclusion using wildcards. Multiple patterns should be separated by commas (`,`).

* **Include only Python and JS**: `OneSource -i "*.py,src/**/*.js"`
* **Exclude venv and logs**: `OneSource -x "venv/**,**/*.log"`

### 3. Custom Markers & Tree Control

* **Custom XML Tags**: Change the default `<file>` tag to something else: `OneSource --marker code`
* **Disable Tree**: Remove the project structure section from the output: `OneSource --no-tree`

### 4. Persistent Configuration

Save your current flags (including complex filters) to avoid re-typing them:

```bash
OneSource --include "src/**" --exclude "tests/**" --marker code --save

```

This creates a `.onesourcerc` file. Subsequent runs only require the command `OneSource`.

---

## Command Line Arguments

| Argument | Flag | Description | Default |
| --- | --- | --- | --- |
| `path` | - | Target project path | `.` |
| `-o` | `--output` | Output filename | `allCode.txt` |
| `-i` | `--include` | Include patterns (e.g., `*.py,src/**/*.js`) | All |
| `-x` | `--exclude` | Exclude patterns (e.g., `venv/**,**/*.log`) | None |
| `-m` | `--marker` | Custom XML tag name | `file` |
| `--no-tree` | - | Disable project structure tree in output | `False` |
| `-c` | `--copy` | Copy the final result to clipboard | `False` |
| `-t` | `--tokens` | Calculate precise tokens (requires `tiktoken`) | `False` |
| `--max-size` | - | Maximum size allowed per file in KB | `500` |
| `--dry-run` | - | List files and tokens without writing to disk | `False` |
| `--save` | - | Save current flags to `.onesourcerc` | `False` |
| `--no-ignore` | - | Ignore `.gitignore` rules | `False` |

---

## Output Format Example

```xml
<project_structure>
project-name/
|-- src/
|   |-- main.py
|   \-- api.py
\-- config.json
</project_structure>

<file path="src/main.py">
# Source code content...
</file>

<file path="config.json">
# Configuration content...
</file>

```

---

## Technical Specifications

* **Robust Encoding**: Uses `utf-8` with error handling to ensure compatibility across Windows (CP950) and Unix environments.
* **Recursion Safety**: Automatically skips symbolic links to prevent infinite loops.
* **Smart Filtering**: Respects `.gitignore` by default and automatically excludes `.git` and the output file itself.
