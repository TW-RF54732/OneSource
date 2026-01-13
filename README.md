Here is the updated **README.md** including a dedicated section for Windows users to utilize the pre-compiled `.exe` CLI tool.

---

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

### 2. Copy to Clipboard

Processes the project and automatically copies the result to your clipboard:

```bash
OneSource -c

```

### 3. Token Preview (Dry Run)

Check the total token count and file list without creating an output file:

```bash
OneSource --dry-run -t

```

### 4. Persistent Configuration

Save specific exclusions or extensions to avoid repeating them in future sessions:

```bash
OneSource --exclude venv,dist --ext .py,.js --save

```

This creates a `.onesourcerc` file in the current directory. Subsequent executions only require the command `OneSource`.

---

## Command Line Arguments

| Argument | Flag | Description | Default |
| --- | --- | --- | --- |
| `path` | - | Target project path | `.` |
| `-o` | `--output` | Output filename | `allCode.txt` |
| `-e` | `--ext` | Filter by file extensions (e.g., `.py,.ts`) | All |
| `-c` | `--copy` | Copy the final result to clipboard | `False` |
| `-t` | `--tokens` | Calculate precise tokens (requires `tiktoken`) | `False` |
| `--exclude` | - | Additional directory/file names to ignore | None |
| `--max-size` | - | Maximum size allowed per file in KB | `500` |
| `--dry-run` | - | List files and tokens without writing to disk | `False` |
| `--save` | - | Save current flags to `.onesourcerc` | `False` |
| `--no-ignore` | - | Ignore `.gitignore` rules | `False` |

---

## Output Format

The output is structured to maximize AI comprehension:

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

* **Recursion Safety**: Automatically skips symbolic links to prevent infinite loops.
* **Encoding**: Forces `utf-8` encoding for all read operations.
* **Self-Exclusion**: Automatically ignores the `.git` directory and the generated output file to prevent recursive bloat.
