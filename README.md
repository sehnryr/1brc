# The One Billion Row Challenge

-   Challenge blog post: https://www.morling.dev/blog/one-billion-row-challenge/
-   Challenge repository: https://github.com/gunnarmorling/1brc

The challenge: **compute simple floating-point math over 1 billion rows. As fast
as possible, without dependencies.**

## Test Hardware

-   **CPU**: AMD Ryzen™ 9 5950X × 32
-   **RAM**: 32 GB DDR4

If the file is read from a disk, it will be read sequencially and bottlenecked
by the speed of the disk. E.g. with a 3,500 MB/s SSD, the theoretical maximum
read time for a 14 GB file is 4.0 seconds.

Bypassing the file system can be done in many ways. The most common way is to
use memory-mapped files using the `mmap` system call. However, this approach
requires a dependency on the `libc` or `memmap` crate, which is not allowed in
the challenge.

The way I chose was to move the file manually to a `tmpfs` partition. This way,
the file is read directly from memory. This approach is not portable, but it's
the fastest way to read a file with the given constraints. The downside is that
the file must fit in memory, which is not always possible.

## Running the challenge

### Creating the sample file and expected output

To create the sample file and the expected output, run the following command:

```bash
cargo run --release --package create-sample 1000000000

# Optionally, move the file to a tmpfs partition
mv sample.txt /tmp
```

> [!WARNING]
> This command will create a file with 1 billion rows, which can take a long
> time will consume 14 GB of disk space. Make sure you have enough disk space
> before running this command. (It takes 2 minutes on my machine.)

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
