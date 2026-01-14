import os
import argparse
import json
from pathlib import Path
import pathspec
import pyperclip
import tiktoken_ext.openai_public
# Optional: Precise Token Calculation
try:
    import tiktoken
    HAS_TIKTOKEN = True
except ImportError:
    HAS_TIKTOKEN = False

BANNER = r"""
==========================================================
  ____  _   _ _____   ____   ___  _   _ ____   ____ _____ 
 / __ \| \ | | ____| / ___| / _ \| | | |  _ \ / ___| ____|
| |  | |  \| |  _|   \___ \| | | | | | | |_) | |   |  _|  
| |__| | |\  | |___   ___) | |_| | |_| |  _ <| |___| |___ 
 \____/|_| \_|_____| |____/ \___/ \___/|_| \_\\____|_____|
                          
 >> DOS-STYLE PROJECT AGGREGATOR | VIBE CODING EDITION <<
==========================================================
"""

CONFIG_FILE = ".onesourcerc"

class OneSource:
    def __init__(self):
        self.args = self._parse_args()
        self.root = Path(self.args.path).resolve()
        self.spec = self._load_gitignore()
        self.output_path = Path(self.args.output)
        self.encoder = tiktoken.get_encoding("cl100k_base") if HAS_TIKTOKEN else None

    def _parse_args(self):
        defaults = {}
        if os.path.exists(CONFIG_FILE):
            try:
                with open(CONFIG_FILE, "r") as f:
                    defaults = json.load(f)
            except: 
                pass

        parser = argparse.ArgumentParser(description=BANNER, formatter_class=argparse.RawDescriptionHelpFormatter)
        parser.add_argument("path", nargs="?", default=defaults.get("path", "."), help="Target project path")
        parser.add_argument("-o", "--output", default=defaults.get("output", "allCode.txt"), help="Output filename")
        parser.add_argument("-e", "--ext", default=defaults.get("ext"), help="Filter by extensions (e.g., .py,.js)")
        parser.add_argument("--exclude", default=defaults.get("exclude"), help="Additional names to exclude")
        parser.add_argument("--max-size", type=int, default=defaults.get("max_size", 500), help="Max file size (KB)")
        parser.add_argument("--no-ignore", action="store_true", help="Ignore .gitignore rules")
        parser.add_argument("--dry-run", action="store_true", help="Preview list without writing to disk")
        parser.add_argument("-c", "--copy", action="store_true", help="Copy output to clipboard")
        parser.add_argument("-t", "--tokens", action="store_true", help="Calculate token count")
        parser.add_argument("--save", action="store_true", help="Save current arguments as default config")

        args = parser.parse_args()

        if args.save:
            config_to_save = {
                "output": args.output,
                "ext": args.ext,
                "exclude": args.exclude,
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
        return pathspec.PathSpec.from_lines('gitwildmatch', gi.read_text().splitlines()) if gi.exists() else None

    def _is_binary(self, path: Path):
        try:
            with open(path, 'tr') as f:
                f.read(1024)
                return False
        except: 
            return True

    def _should_ignore(self, path: Path):
        if path.is_symlink() or ".git" in path.parts or path == self.output_path: 
            return True
        rel_path = path.relative_to(self.root)
        if self.args.exclude and any(ex in path.parts for ex in self.args.exclude.split(",")): 
            return True
        if self.spec and self.spec.match_file(str(rel_path)): 
            return True
        if self.args.ext and path.suffix not in self.args.ext.split(","): 
            return True
        if path.is_file():
            if path.stat().st_size > self.args.max_size * 1024 or self._is_binary(path): 
                return True
        return False

    def _generate_tree(self, dir_path, prefix=""):
        tree_str = ""
        entries = sorted([e for e in dir_path.iterdir() if not self._should_ignore(e)], key=lambda x: (x.is_file(), x.name))
        for i, entry in enumerate(entries):
            is_last = (i == len(entries) - 1)
            connector = "\\-- " if is_last else "|-- "
            tree_str += f"{prefix}{connector}{entry.name}\n"
            if entry.is_dir():
                tree_str += self._generate_tree(entry, prefix + ("    " if is_last else "|   "))
        return tree_str

    def run(self):
        # Display Banner
        print(BANNER)
        
        mode_label = "[DRY RUN]" if self.args.dry_run else "[PROCESSING]"
        print(f"{mode_label} Root: {self.root}")

        valid_files = [p for p in self.root.rglob("*") if p.is_file() and not self._should_ignore(p)]
        total_tokens = 0
        
        out_file = None
        if not self.args.dry_run:
            out_file = open(self.output_path, "w", encoding="utf-8")
            out_file.write(f"<project_structure>\n{self.root.name}/\n{self._generate_tree(self.root)}</project_structure>\n\n")

        for p in valid_files:
            rel_path = p.relative_to(self.root)
            try:
                content = p.read_text(encoding="utf-8")
                if self.args.tokens and HAS_TIKTOKEN:
                    total_tokens += len(self.encoder.encode(content))
                
                if out_file:
                    out_file.write(f'<file path="{rel_path}">\n{content}\n</file>\n\n')
                
                print(f"  + {rel_path}")
            except Exception as e:
                print(f"  ! Error reading {rel_path}: {e}")

        if out_file: 
            out_file.close()

        print("\n" + "="*40)
        print(f"Files Processed: {len(valid_files)}")
        if self.args.tokens:
            token_str = f"{total_tokens:,}" if HAS_TIKTOKEN else "tiktoken not installed"
            print(f"Total Tokens:    {token_str}")
        
        if not self.args.dry_run:
            print(f"Output saved to: {self.output_path}")
            if self.args.copy:
                pyperclip.copy(self.output_path.read_text(encoding="utf-8"))
                print("Copied to clipboard.")
        print("="*40)

if __name__ == "__main__":
    OneSource().run()