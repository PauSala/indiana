# Mole

```console
Searches recursively for a specified cargo dependency in all projects within a given directory

Usage: mole [OPTIONS] <NAME>

Arguments:
  <NAME>
          The name of the dependency to search for

Options:
  -p, --path <PATH>
          The directory to search in. Defaults to the current directory

          [default: .]

  -d, --deep
          Flag to indicate whether to search for the dependency in Cargo.lock as well

  -t, --threaded
          Flag to indicate whether to explore files in parallel

  -f, --filter <FILTER>
          Semver filter to filter the dependency by. Accepts a single semver version or a range in quotes, coma separated.

          Example: ">= 1.0.0, < 2.0.0"

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
