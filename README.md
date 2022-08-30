# json2yaml / yaml2json

Simple tool to convert between json and yaml using the excellent serde library.

## Installation

Clone this repository and install locally with the following command:
```shell
$ cargo install --path .
```

## Usage

Both `json2yaml` and `yaml2json` convert from standard input to standard output.

```shell
$ json2yaml < some.json > some.yaml
$ yaml2json < some.yaml > some.json
```
