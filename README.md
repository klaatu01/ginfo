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
ginfo test.gz
```

```shell
echo "H4sIAAAAAAAAA8vPUMhIzcnJVyjJSC1KBQBvyKZBDgAAAA==" | ginfo -b
```

```shell
ginfo -b test.b64
```
