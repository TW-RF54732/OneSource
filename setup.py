from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="onesource-cli",
    version="1.1.3",
    author="lolLeo",
    author_email="your.email@example.com",
    description="A vibe coding tool to aggregate project code for LLMs.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/TW-RF54732/OneSource",
    packages=find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires='>=3.8',
    install_requires=[
        "pathspec",
        "pyperclip",
        "tiktoken", 
    ],
    entry_points={
        "console_scripts": [
            "OneSource=onesource.main:main", 
        ],
    },
)