import os
import sys
import shutil
import argparse
import json
from pathlib import Path
import pathspec
import pyperclip

# Optional: Precise Token Calculation
try:
    import tiktoken
    import tiktoken_ext.openai_public 
    HAS_TIKTOKEN = True
except ImportError:
    HAS_TIKTOKEN = False

VERSION = "v1.2.0"

# This BANNER will appear in --help and the Installer
BANNER = rf"""
==========================================================
  ____  _   _ _____   ____   ___  _   _ ____   ____ _____ 
 / __ \| \ | | ____| / ___| / _ \| | | |  _ \ / ___| ____|
| |  | |  \| |  _|   \___ \| | | | | | | |_) | |   |  _|  
| |__| | |\  | |___   ___) | |_| | |_| |  _ <| |___| |___ 
 \____/|_| \_|_____| |____/ \___/ \___/|_| \_\\____|_____|
                          
 >> OneSource {VERSION} | The Local-First Vibe Coding Tool <<
==========================================================
"""

CONFIG_FILE = ".onesourcerc"

class OneSource:
    def __init__(self):
        self.args = self._parse_args()
        self.root = Path(self.args.path).resolve()
        # Resolve absolute path to prevent the output file from checking itself improperly
        self.output_path = Path(self.args.output).resolve()
        
        self.spec = self._load_gitignore()
        
        # Build pathspecs for include/exclude
        self.include_spec = self._build_pathspec(self.args.include)
        self.exclude_spec = self._build_pathspec(self.args.exclude)
        
        self.encoder = None
        if HAS_TIKTOKEN:
            try:
                self.encoder = tiktoken.get_encoding("cl100k_base")
            except Exception as e:
                # Keep silent in CLI
                pass

    def _build_pathspec(self, patterns_str):
        if not patterns_str:
            return None
        patterns = [p.strip() for p in patterns_str.split(",") if p.strip()]
        return pathspec.PathSpec.from_lines('gitwildmatch', patterns)

    def _parse_args(self):
        defaults = {}
        if os.path.exists(CONFIG_FILE):
            try:
                with open(CONFIG_FILE, "r") as f:
                    defaults = json.load(f)
            except: 
                pass

        # Setting description=BANNER here ensures it shows up when running --help
        parser = argparse.ArgumentParser(
            description=BANNER, 
            formatter_class=argparse.RawDescriptionHelpFormatter
        )
        
        parser.add_argument("path", nargs="?", default=defaults.get("path", "."), help="Target project path")
        parser.add_argument("-o", "--output", default=defaults.get("output", "allCode.txt"), help="Output filename")
        parser.add_argument("-i", "--include", default=defaults.get("include"), help="Include patterns (e.g., *.py,src/**/*.js)")
        parser.add_argument("-x", "--exclude", default=defaults.get("exclude"), help="Exclude patterns (e.g., venv/,**/*.log)")
        parser.add_argument("-m", "--marker", default=defaults.get("marker", "file"), help="Custom XML tag name (default: file)")
        parser.add_argument("--no-tree", action="store_true", default=defaults.get("no_tree", False), help="Disable project structure tree")
        parser.add_argument("--max-size", type=int, default=defaults.get("max_size", 500), help="Max file size (KB)")
        parser.add_argument("--no-ignore", action="store_true", help="Ignore .gitignore rules")
        parser.add_argument("--dry-run", action="store_true", help="Preview list without writing to disk")
        parser.add_argument("-c", "--copy", action="store_true", help="Copy output to clipboard")
        parser.add_argument("-t", "--tokens", action="store_true", help="Calculate token count")
        parser.add_argument("--save", action="store_true", help="Save current arguments as default config")
        
        # Add a version flag
        parser.add_argument("-v", "--version", action="version", version=f"OneSource {VERSION}")

        args = parser.parse_args()

        if args.save:
            config_to_save = {
                "output": args.output,
                "include": args.include,
                "exclude": args.exclude,
                "marker": args.marker,
                "no_tree": args.no_tree,
                "max_size": args.max_size
            }
            with open(CONFIG_FILE, "w") as f:
                json.dump(config_to_save, f, indent=4)
            print(f"[*] Configuration saved to {CONFIG_FILE}")

        return args

    def _load_gitignore(self):
        if self.args.no_ignore: 
            return None
        gi = self.root / ".gitignore"
        if gi.exists():
            try:
                # Read gitignore with utf-8
                content = gi.read_text(encoding="utf-8", errors="ignore")
                return pathspec.PathSpec.from_lines('gitwildmatch', content.splitlines())
            except Exception as e:
                print(f"  ! Warning: Failed to read .gitignore: {e}")
                return None
        return None

    def _is_binary(self, path: Path):
        # Check for binary by attempting to read as UTF-8 first
        try:
            with open(path, 'r', encoding='utf-8') as f:
                f.read(1024)
                return False
        except UnicodeDecodeError:
            return True # Truly binary or non-UTF-8
        except Exception: 
            return True # Other read errors, treat as binary

    def _should_ignore(self, path: Path):
        # Compare absolute paths
        if path.is_symlink() or ".git" in path.parts or path == self.output_path: 
            return True
        
        # Normalize path separators to forward slashes for pathspec compatibility
        rel_path = str(path.relative_to(self.root)).replace('\\', '/')
        
        # Gitignore check
        if self.spec and self.spec.match_file(rel_path): 
            return True
        
        # Custom exclude check
        if self.exclude_spec and self.exclude_spec.match_file(rel_path):
            return True

        if path.is_file():
            # Custom include check
            if self.include_spec and not self.include_spec.match_file(rel_path):
                return True
            
            # Size and Binary check
            if path.stat().st_size > self.args.max_size * 1024 or self._is_binary(path): 
                return True
                
        return False

    def _generate_tree(self, dir_path, prefix=""):
        tree_str = ""
        try:
            entries = sorted([e for e in dir_path.iterdir() if not self._should_ignore(e)], 
                            key=lambda x: (x.is_file(), x.name))
        except PermissionError:
            return f"{prefix}[Permission Denied]\n"

        for i, entry in enumerate(entries):
            is_last = (i == len(entries) - 1)
            connector = "\\-- " if is_last else "|-- "
            tree_str += f"{prefix}{connector}{entry.name}\n"
            if entry.is_dir():
                tree_str += self._generate_tree(entry, prefix + ("    " if is_last else "|   "))
        return tree_str

    def run(self):
        # NOTE: BANNER is NOT printed here anymore to keep CLI output clean.
        
        mode_label = "[DRY RUN]" if self.args.dry_run else "[PROCESSING]"
        print(f"{mode_label} Root: {self.root}")

        valid_files = [p for p in self.root.rglob("*") if p.is_file() and not self._should_ignore(p)]
        
        project_tree = None
        if not self.args.no_tree:
            project_tree = f"{self.root.name}/\n{self._generate_tree(self.root)}"
            print("\nProject Structure Preview:")
            print("-" * 20)
            print(project_tree)
            print("-" * 20 + "\n")

        total_tokens = 0
        out_file = None
        
        if not self.args.dry_run:
            out_file = open(self.output_path, "w", encoding="utf-8")
            if project_tree:
                out_file.write(f"<project_structure>\n{project_tree}</project_structure>\n\n")

        marker = self.args.marker
        for p in valid_files:
            rel_path = str(p.relative_to(self.root)).replace('\\', '/')
            try:
                # Use utf-8 and replace errors to avoid crashing on weird characters
                content = p.read_text(encoding="utf-8", errors="replace")
                
                if self.args.tokens and self.encoder:
                    total_tokens += len(self.encoder.encode(content))
                
                if out_file:
                    out_file.write(f'<{marker} path="{rel_path}">\n{content}\n</{marker}>\n\n')
                
                print(f"  + {rel_path}")
            except Exception as e:
                print(f"  ! Error reading {rel_path}: {e}")

        if out_file: 
            out_file.close()

        print("\n" + "="*40)
        print(f"Files Processed: {len(valid_files)}")
        if self.args.tokens:
            token_str = f"{total_tokens:,}" if self.encoder else "tiktoken error"
            print(f"Total Tokens:    {token_str}")
        
        if not self.args.dry_run:
            print(f"Output saved to: {self.output_path}")
            if self.args.copy:
                try:
                    pyperclip.copy(self.output_path.read_text(encoding="utf-8"))
                    print("Copied to clipboard.")
                except Exception as e:
                    print(f"Clipboard error: {e}")
        print("="*40)

def main():
    OneSource().run()

if __name__ == "__main__":
    main()