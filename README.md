# regularize-filename

This small Rust command-line application will accept any number of files and rename them according to your favourite naming convention.

## Usage

```txt
regularize-filename 0.1.0
Regularize the names of your files according to your favourite naming convention

USAGE:
    regularize-filename [FLAGS] [FILE]...

FLAGS:
    -c, --camel-case     Use camelCase
    -h, --help           Prints help information
    -k, --kebab-case     Use kebab-case
    -p, --pascal-case    Use PascalCase
    -s, --snake-case     Use snake_case
    -V, --version        Prints version information

ARGS:
    <FILE>...    Files to process
```

## License

[MIT](LICENSE)
