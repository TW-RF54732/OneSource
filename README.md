# OneSource

OneSource is a high-efficiency project code aggregator designed specifically for **Vibe Coding**. It consolidates source code from any project into a single structured text file, making it easy to provide full project context to Large Language Models (LLMs) in one step.

---

## Core Features

* **Streaming I/O Architecture**: Processes files using a stream-based mechanism. This ensures a minimal memory footprint regardless of project size, preventing Out-of-Memory (OOM) errors.
* **Precise Token Counting**: Integrates with the `tiktoken` library to provide exact token statistics (cl100k_base), helping you manage LLM context window limits effectively.
* **General Purpose Filtering**:
* **Gitignore Support**: Automatically respects rules defined in `.gitignore`.
* **Binary Detection**: Identifies and skips non-text files (images, executables, etc.).
* **Size Protection**: Prevents large logs or datasets from being included via the `--max-size` flag.


* **LLM-Optimized Structure**: Uses XML tags (`<file path="...">`) to wrap code blocks, providing the most reliable boundary recognition for models like Claude 3.5 and GPT-4.
* **Configuration Persistence**: Supports `.onesourcerc` files to store your preferred settings, eliminating the need to re-enter arguments for every run.

---

## Installation

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
python app.py

```

### 2. Copy to Clipboard

Processes the project and automatically copies the result to your clipboard:

```bash
python app.py -c

```

### 3. Token Preview (Dry Run)

Check the total token count and file list without creating an output file:

```bash
python app.py --dry-run -t

```

### 4. Persistent Configuration

Save specific exclusions or extensions to avoid repeating them in future sessions:

```bash
python app.py --exclude venv,dist --ext .py,.js --save

```

This creates a `.onesourcerc` file in the current directory. Subsequent executions only require the command `python app.py`.

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
