# v1.2.0

## 🚀 New Features
* ## 🐛 Bug Fixes
* ## 📦 Installation
Download the `OneSource_Windows_Installer.zip` below.
# v1.2.0: 

This release focuses on **Distribution & Experience**. We've completely overhauled how Windows users install OneSource, separating the installation logic from the core tool to ensure maximum stability and flexibility.

## 🚀 New Features

### 🪟 3 Ways to Install on Windows

We realized one size doesn't fit all. We now offer three distinct installation methods for Windows users:

1. **Local Installer (`.zip`)**: The recommended way. Download, unzip, and run `install.bat`. It handles PATH registration automatically.
2. **Network Installer (PowerShell)**: A single one-line command to download and install the latest version instantly.
```powershell
irm https://raw.githubusercontent.com/TW-RF54732/OneSource/main/install.ps1 | iex

```


3. **Portable Mode**: Just want the raw `.exe`? You got it. No strings attached.

### 📚 Documentation Overhaul

* **Collapsible Sections**: The `README.md` now features clean, collapsible dropdowns for installation instructions.
* **Platform Segmentation**: Instructions for Windows and Python/Linux/macOS are now clearly separated to reduce confusion.

## 🛠️ Technical Improvements

* **Pure Core Logic**: Removed the experimental "Self-Installing" logic from `main.py`. The core executable is now 100% focused on project packing, while installation is handled by dedicated external scripts (`install.bat` / `install.ps1`).
* **Manifest Updates**: Optimized `MANIFEST.in` to ensure a clean PyPI package distribution without unnecessary build artifacts.

## 📦 How to Update

* **Windows**: Download the new **`OneSource_Windows_Installer.zip`** from Assets.
* **Python Users**:
```bash
pip install --upgrade onesource-cli

```