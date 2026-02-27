# Jito Shank CLI

A command-line interface tool for managing and generating IDL files for Solana programs using the shank-idl library.

## Overview

Jito Shank CLI simplifies the process of generating IDL files.
It allows you to extract IDL definitions from multiple module paths and combine them into a single comprehensive IDL file.

## Installation

```bash
cargo install jito-shank-cli
```

## Getting Started

### Generate IDL

### Command Structure

The CLI supports the following commands and options:

#### Global Options

- `--program-env-path`: Path to the environment file containing program IDs
- `--output-idl-path`: Directory where the generated IDL file will be saved

#### Commands

##### Generate

Generates an IDL file.

```bash
shank-cli --program-env-path ./.env --output-idl-path ./idl generate --program-env-key MY_PROGRAM_ID --idl-name my_program --module-paths core program sdk
```

###### Options

- `--program-env-key`: Key in the environment file that contains the program ID
- `--idl-name`: Name for the generated IDL file
- `--module-paths`: One or more paths to Rust modules containing shank annotations

## Environment File Format

The environment file should contain key-value pairs:

```
PROGRAM_ID=
```
