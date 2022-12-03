# Advent of Code Rust Solutions

This repository contains the solutions I used for Advent of Code, written in Rust.

Note that I may not be able to finish all the challenges in Advent of Code.

Special thanks to [B3NNY/Ben-Lichtman](https://github.com/Ben-Lichtman) for [aoc_driver](https://github.com/Ben-Lichtman/aoc_driver)

This repository also contains a CLI, made using [aoc_driver](https://github.com/Ben-Lichtman/aoc_driver) that allows me to run each solution without changine what runs manually.

The usage is as follows:

```sh
aoc-rs <year> <day> <part> [-p]
```

The flag `-p` must be put after the year, day, and part and will automatically submit the calculated answer.

Note that Advent of Code enforces some rate limits so it is advisable to not spam it.

The CLI is far from user-proof and may crash.

Downloading of inputs and submission of answers is handled by [aoc_driver](https://github.com/Ben-Lichtman/aoc_driver)
