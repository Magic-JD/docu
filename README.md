# Docu
Docu is a CLI tool for easy documentation of mini scripts (scriptlets) you write. These scriptlets might be useful, but not significant enough to become an alias or standalone script. Docu lets you save, categorize, and search your scriptlets in neat Markdown documentation.

## Features
- Store one-off scriptlets with a name and description
- List stored scriptlets with filters by tool or keyword
- Search by description to find matching scriptlets quickly
- Output stored commands with placeholders for context variables

## Installing the program ![Latest Release](https://img.shields.io/github/v/release/Magic-JD/docu?include_prereleases)


### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Magic-JD/docu/releases/latest/download/docu-installer.sh | sh
```

### Install prebuilt binaries via Homebrew

```sh
brew install magic-jd/tap/docu
```

### Install latest from source with cargo:

```sh
cargo install --git https://github.com/Magic-JD/docu.git
```

## Table of Contents

1. [Usage](#usage)
    1. [Add a scriptlet](#add-a-scriptlet)
    2. [Show all scriptlets](#show-all-scriptlets)
    3. [Filter by command](#filter-by-command)
    4. [Search by description](#search-by-description)
2. [Useful aliases](#useful-aliases)
3. [Configuration](#configuration)
    1. [Colors](#colors)
4. [Contributing](#contributing)
5. [License](#license)

## Usage
### Add a scriptlet
```bash
docu add [OPTIONS] "SCRIPT"
```
- SCRIPT: The raw command or pipeline to store.
- You will be prompted to enter:
- Name: A short identifier (e.g. camel case header)
- Description: A sentence summarizing the purpose.

#### Example

```bash
docu add "sed -i '1{s/ /_/g}' products.csv"
```

When prompted:
- Name: camel case header
- Description: Convert spaces in CSV header to underscores

### Show all scriptlets
```bash
docu show
```
Lists every stored scriptlet with its name and description.

### Filter by command
```bash
docu show $TOOL_NAME
```
Displays only scriptlets containing the sed command. 

#### Example
Show all sed scriptlets
```bash
docu show sed
```

### Search by description
```bash
docu search $SEARCH_TERM
```
Returns an ordered list of scriptlets matching the search terms in their descriptions.

#### Examples

Find by keyword
```bash
docu search "camel case"
```

## Useful aliases

If you add this to your aliases:

```bash
alias docl='docu add "$(fc -ln -1)"'
```

Then running docl will automatically run for the last command you entered.

#### Example

```bash
cat demo.txt | sort | uniq -c | awk '$1 == 1 {print $2}'
docl # This will then run docu add with the above command, allowing you to save it.
```

## Configuration

You can generate a copy of the default config to see the configuration options using the command:

```bash
docu generate-config
```

This will print out the location of the created file, which will be:

- Linux: ~/.config/is-fast/config.toml
- macOS: ~/Library/Application Support/is-fast/config.toml
- Windows: %APPDATA%\is-fast\config.toml

### Colors

```toml
[colors]
scriptlet_name = "yellow"
scriptlet_description = "white"
```

You can use color names, hex codes, or RGB values.

The available colors are:
- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white

You can also use hex codes (e.g., `#RRGGBB`) or RGB values (e.g., `rgb(r, g, b)`).

## Contributing
1. Fork the repository.
2. Create a feature branch: git checkout -b feature-name.
3. Commit your changes: git commit -m "Add feature".
4. Push to your branch: git push origin feature-name.
5. Open a pull request.

## License
MIT License. See LICENSE for details.