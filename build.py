import PyInstaller.__main__
import os

def build():
    params = [
        'app.py',
        '--onefile',
        '--console',
        '--name=OneSource',
        '--clean',
        '--collect-all=tiktoken',
        '--copy-metadata=tiktoken',
        '--collect-all=pathspec',
    ]
    
    PyInstaller.__main__.run(params)

if __name__ == "__main__":
    build()