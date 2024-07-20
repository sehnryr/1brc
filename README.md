# The One Billion Row Challenge

-   Challenge blog post: https://www.morling.dev/blog/one-billion-row-challenge/
-   Challenge repository: https://github.com/gunnarmorling/1brc

The challenge: **compute simple floating-point math over 1 billion rows. As fast
as possible, without dependencies.**

## Running the challenge

### Creating the sample file and expected output

To create the sample file and the expected output, run the following command:

```bash
cargo run --release --package create-sample 1000000000
```

> [!WARNING]
> This command will create a file with 1 billion rows, which can take a long
> time will consume 14 GB of disk space. Make sure you have enough disk space
> before running this command. (It took 2 minutes on my machine.)

### Running the challenge

To run the challenge, use the following command:

```bash
cargo build --release --package onebrc
time ./target/release/onebrc sample.txt > /dev/null
```

### Validating the result

To validate the result of the computation, you can use the following command:

```bash
cargo run --release --package onebrc sample.txt | diff expected.txt -
```

> [!NOTE]
> Rust sorts unicode by codepoint, so the output may not be in the same order as
> some other languages.
