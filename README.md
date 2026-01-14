# OneSource ⚡

> **The "Vibe Coding" CLI for Windows & Python Developers.**
>
> **No Node.js? No Python? No Problem.** Just download the `.exe` and start vibing with your AI.

![License](https://img.shields.io/badge/license-MIT-blue.svg) ![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

OneSource is a lightweight tool designed to make **AI-assisted coding** effortless. It packs your entire project structure and code into a single context-rich text file (or copies it to your clipboard), so you can feed it to ChatGPT, Claude, or Gemini in one second.

---

## 🚀 Why Use OneSource?

* **⚡ Zero Dependencies (Windows):** No `npm install`, no `pip install`. Just a single executable file.
* **📋 Clipboard Ready:** With the `-c` flag, your codebase is copied to your clipboard instantly.
* **🧠 Context Aware:** Automatically respects `.gitignore` to keep trash files out of your LLM context.
* **🌳 Structure Visualization:** Includes a tree view of your project so the AI understands your architecture.

---

## 📥 Installation

### 🖥️ For Windows Users (The Easiest Way)

You don't need Python or Node.js installed.

1.  **Download**: Get the latest `OneSource.exe` from the **[Releases Page]** (link-to-releases).
2.  **Place It**: Move the file to a folder, e.g., `C:\Tools\`.
3.  **Set Up (Once)**: Add it to your system PATH so you can run it from anywhere.
    * Press `Win` key, type **"env"**, select **"Edit the system environment variables"**.
    * Click **"Environment Variables"** -> Under **"User variables"**, find **`Path`** -> **"Edit"**.
    * Click **"New"** -> Paste the folder path (e.g., `C:\Tools\`).
    * Click **OK** on all windows.
4.  **Verify**: Open a new Command Prompt (cmd) and type `OneSource`.

### 🐍 For Python Users

If you prefer `pip` or are on Linux/macOS:

```bash
pip install pathspec pyperclip tiktoken
# Then run the script directly
python app.py