# Vision File Renamer

Remove leading hashes from exported Prelude xml files and replace them with the directory name.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
vision-file-renamer [OPTIONS] <DIRECTORY>
```

### Arguments

- <DIRECTORY>  Path to the directory that contains the files to rename

### OPTIONS

- -p, --prefix-length <PREFIX_LENGTH>: The length ofthe prefix to replace
- -r, --recursive: Recursively traverse directories
- -n, --no-append: Don't append the parent directory name to the file name
- -v, --verbose: Verrbose output
- -h, --help: Print help
- -V, --version: Print version

## Contributing

Contributions to this project are welcome. If you are interesting in contributing please see our [contributing guide](CONTRIBUTING.md)
