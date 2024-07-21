# The One Billion Row Challenge

-   Challenge blog post: https://www.morling.dev/blog/one-billion-row-challenge/
-   Challenge repository: https://github.com/gunnarmorling/1brc

The challenge: **compute simple floating-point math over 1 billion rows. As fast
as possible, without dependencies.**

## Test Hardware

-   **CPU**: AMD Ryzen™ 7 5700X × 16
-   **RAM**: 16 GB DDR4
-   **SSD**: PNY CS3030 1TB SSD (3,500 MB/s Seq. Read)

The theoretical maximum read time for a 14 GB file (the sample file) is 4.0
seconds.

With my last implementation, the time to read the file was ~4.5 seconds. It's
very close to the theoretical maximum, so I don't think I can improve the read
time any further without changing either the file format or the hardware.

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
