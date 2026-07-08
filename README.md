# weez

A fast `wc`-style line, word, and character counter with stdin support.

## Installation

```
cargo install weez
```

## Usage

Count lines, words, and characters in a file:

```
weez file.txt
```

Read from standard input via a pipe:

```
cat file.txt | weez
echo "hello world" | weez
```

By default `weez` prints all three counts. Use flags to show only what you want — flags are additive and stackable:

```
weez -l file.txt        # lines only
weez -w file.txt        # words only
weez -lw file.txt       # lines and words
```

## Flags

| Flag              | Description      |
| ----------------- | ---------------- |
| `-l`, `--lines`   | Count lines      |
| `-w`, `--words`   | Count words      |
| `-c`, `--chars`   | Count characters |
| `-h`, `--help`    | Show help        |
| `-V`, `--version` | Show version     |

## License

Licensed under MIT.
