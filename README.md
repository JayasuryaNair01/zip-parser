# zip-parser

A ZIP file parser in Rust, written from scratch with no ZIP libraries. I'm building it to understand how the format works at the byte level.

## Status: work in progress

What it does now:
- Finds the End of Central Directory record by scanning backwards for its signature
- Reads the file count and the central directory offset
- Parses the first central directory entry: file name, compressed and uncompressed size, local header offset
- Follows the local header to the file data and extracts it (stored, uncompressed files only)

Next:
- Loop over every entry, not just the first
- Deflate decompression
- Bounds-checked reads so malformed archives fail cleanly instead of panicking
- Zip slip protection (block entries like `../../file` writing outside the output folder)

## Run

```
cargo run
```

Expects a `test.zip` in the project folder for now.

## Reference

[PKWARE APPNOTE.TXT](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT), the ZIP format specification.
