# OneSource ⚡

> **The Local-First Project Packer for AI Context.**
>
> **Escape the Node.js ecosystem.** No `npm install`. No file uploads.
> **Just download and run.** (Or `pip install` if you prefer).

[![PyPI version](https://img.shields.io/pypi/v/onesource-cli.svg)](https://pypi.org/project/onesource-cli/)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

**OneSource** aggregates your entire project into a single, context-rich text file (or clipboard) for LLMs like Claude, ChatGPT, and Gemini.

It bridges the gap between **Windows users** who want a simple `.exe` and **Python developers** who want a native CLI tool.

---

## 🥊 Why OneSource? (vs The Rest)

| Feature | **OneSource** ⚡ | **Repomix** (Repopack) | **Gitingest** | **code2prompt** |
| :--- | :--- | :--- | :--- | :--- |
| **No Node.js Required** | ✅ **YES** (Standalone EXE) | ❌ No (Need NPM) | ✅ Yes (Web) | ✅ Yes (Rust) |
| **Local Privacy** | ✅ **100% Local** | ✅ Local | ❌ **Uploads/Git Push needed** | ✅ Local |
| **Windows Friendly** | ✅ **Native .exe** | ❌ Complex setup | ✅ Web browser | ⚠️ CLI focused |
| **Edit/Extend** | ✅ **Simple Python** | ❌ TypeScript | ❌ Web Service | ❌ Rust (Harder to mod) |
| **Clipboard Auto-Copy**| ✅ **Built-in** | ✅ Yes | ❌ Manual copy | ✅ Yes |

* **vs Repomix:** Stop installing 200MB of `node_modules` just to pack a text file. OneSource is lightweight.
* **vs Gitingest:** Don't push your private secrets or messy WIP code to GitHub just to analyze it. OneSource works on your *local* disk, offline.
* **vs code2prompt:** Easier for Python developers to customize and integrate into their own scripts.

---

## 📥 Installation

Choose the method that fits your workflow.

### 🅰️ Method A: The "It Just Works" Way (Recommended for Windows)
**Perfect for:** PMs, Students, Windows Users, or "Vibe Coders" who don't want to manage environments.

> **No Python? No Node.js? No Problem.**

1.  **Download**: Get the latest `OneSource.exe` from the **[Releases Page](../../releases)**.
2.  **Run**: Open cmd/PowerShell in your project folder and run `OneSource.exe`.
3.  **(Optional) Add to PATH**: Move it to `C:\Windows\` or any PATH folder to run it from anywhere.

---

### 🅱️ Method B: The Developer Way (Python Native)
**Perfect for:** Python devs, Linux/macOS users, or CI/CD pipelines.

If you already have Python installed, grab it via PyPI:

```bash
pip install onesource-cli

```

---

## 🎮 Usage Scenarios

Run these commands in your project root.

### Scenario 1: The "Lazy" Mode (Bug Fixing) 🌟

You broke the code. You need AI help NOW.
This packs everything (respecting `.gitignore`) and copies it to your clipboard.

```bash
OneSource -c

```

*-> Ctrl+V into ChatGPT.*

### Scenario 2: Focused Backend Work

Don't confuse the AI with frontend assets. Only grab the Python logic.

```bash
OneSource -i "*.py" -c

```

### Scenario 3: "Will this fit in the context window?"

Check token count before pasting.

```bash
OneSource -t --dry-run

```

### Scenario 4: Set It and Forget It

Always exclude `tests/` and `legacy/` folders? Save your config.

```bash
OneSource -x "tests/**,legacy/**" --save

```

*Creates a hidden config file. Next time, just run `OneSource`.*

---

## 📖 Command Reference

| Argument | Description | Default |
| --- | --- | --- |
| `path` | **(Positional)** Target project path. | Current folder (`.`) |
| `-o`, `--output` | Output filename. | `allCode.txt` |
| `-c`, `--copy` | **Auto-copy** result to clipboard. | `False` |
| `-i`, `--include` | Only include files matching this pattern (Applied **AFTER** `.gitignore`). | All non-ignored files |
| `-x`, `--exclude` | Extra patterns to ignore. Wins over  `-i`  if conflict. | `None` |
| `-t`, `--tokens` | Show token count (requires `tiktoken`). | `False` |
| `--no-tree` | Disable the directory tree visualization at the top. | `False` |
| `--max-size` | Skip files larger than this size (in KB). | `500` KB |
| `--no-ignore` | **Unlock mode:** Force scan files even if listed in `.gitignore`. | `False` |
| `--marker` | Custom XML tag for wrapping code (e.g., use `code` instead of `file`). | `file` |
| `--dry-run` | Preview which files will be processed without writing/copying. | `False` |
| `--save` | Save current flags as default config (`.onesourcerc`). | `False` |

---

*Built for Vibe Coding. Privacy First. Local First.*

