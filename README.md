# OneSource ⚡

> **The Local-First Project Packer for AI Context.**
>
> 🚫 **Escape the Node.js ecosystem.** No `npm install`. No file uploads.
> 🚀 **Just download and run.** (Or `pip install` if you prefer).

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

Select your platform below to see the instructions.

<details>
<summary><strong>🪟 Windows Users (Click to expand)</strong></summary>

We offer three ways to install OneSource on Windows. Choose the one that fits your style.

#### Option 1: The Local Installer (Recommended)
*Best for most users. Ensures the tool is added to your system PATH.*

1.  Download `OneSource_Windows_Installer.zip` from the **[Releases Page](https://github.com/TW-RF54732/OneSource/releases)**.
2.  **Unzip** the file.
3.  Double-click `install.bat`.
4.  Done! You can now type `OneSource` in any terminal.

#### Option 2: The Network Installer (PowerShell)
*Best for power users or scripts. Installs the latest version via one command.*

Open **PowerShell** and paste the following:

```powershell
irm [https://raw.githubusercontent.com/TW-RF54732/OneSource/main/install.ps1](https://raw.githubusercontent.com/TW-RF54732/OneSource/main/install.ps1) | iex

```

#### Option 3: The Portable EXE

*Best for USB drives or temporary use.*

1. Download the standalone `OneSource.exe` from Releases.
2. Place it anywhere (e.g., inside your project folder).
3. Run it directly via terminal: `.\OneSource.exe`

</details>

<details>
<summary><strong>🐍 Python Developers / Linux / macOS (Click to expand)</strong></summary>

If you have Python installed or want to integrate this into your CI/CD pipeline, use PyPI.

**Installation:**

```bash
pip install onesource-cli

```

**Upgrade:**

```bash
pip install --upgrade onesource-cli

```

</details>

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
| `-x`, `--exclude` | Extra patterns to ignore. **Wins over `-i**` if there is a conflict. | `None` |
| `-t`, `--tokens` | Show token count (requires `tiktoken`). | `False` |
| `--no-tree` | Disable the directory tree visualization at the top. | `False` |
| `--max-size` | Skip files larger than this size (in KB). | `500` KB |
| `--no-ignore` | **Unlock mode:** Force scan files even if listed in `.gitignore`. | `False` |
| `--marker` | Custom XML tag for wrapping code (e.g., use `code` instead of `file`). | `file` |
| `--dry-run` | Preview which files will be processed without writing/copying. | `False` |
| `--save` | Save current flags as default config (`.onesourcerc`). | `False` |

---

*Built for Vibe Coding. Privacy First. Local First.*