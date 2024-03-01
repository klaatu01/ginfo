# ginfo

`ginfo` is a simple utility to display information about a GZip header.

It can read from a file or from stdin, and can also read base64 encoded input.

It will display the compression method, flags, modification time, and OS of the GZip file.

## Installation

```shell
cargo install ginfo
```

## Examples

```shell
▶ ginfo --help
Usage: ginfo [OPTIONS] [FILE]

Arguments:
  [FILE]  The filename to read from. If not provided, read from stdin

Options:
  -b, --base64  Read the input as base64 encoded
  -h, --help    Print help
```

```shell
▶ ginfo test.gz
Valid GZip file.
Compression Method: 8
Flags: 00001000
Modification Time: 2024-02-21 14:35:35
OS: Unix
```

```shell
▶ ginfo -- test.gz
```

```shell
▶ echo "H4sIAAAAAAAAA8vPUMhIzcnJVyjJSC1KBQBvyKZBDgAAAA==" | ginfo -b
```

```shell
▶ ginfo -b test.b64
```
