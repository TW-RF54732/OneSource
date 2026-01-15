# v1.2.1: The "Clean Slate" Update

This release removes the broken local installation method found in v1.2.0 and streamlines the distribution.

## 📦 Changes

* **Removed Legacy Installer**: The faulty `OneSource_Windows_Installer.zip` and `install.bat` have been removed. 
* **Simplified Windows Installation**: 
    * Use the **Network Installer** (PowerShell) for a full installation.
    * Use the **Portable EXE** for a standalone experience.

## 🛠️ Fixes

* Fixed release artifacts to ensure only valid, working executables are distributed.

## 📥 Installation (Windows)

**Network Install (Recommended):**
```powershell
irm [https://raw.githubusercontent.com/TW-RF54732/OneSource/main/install.ps1](https://raw.githubusercontent.com/TW-RF54732/OneSource/main/install.ps1) | iex
```