# GitHub Tools

## Overview

This tool is a simple cli application that allows you to fetch some basic information about a GitHub repository or user.

## Installation

To use this tool, you can simply clone the repository and run the following command:

1. Clone the repository:

```
git clone https://github.com/golrice/githubtools.git
```

2. Use cargo to build the tool:

```
cd githubtools
cargo build --release
```

## Usage

we have some commands to fetch information about a GitHub repository or user:

1.  `repo`: fetch information about a GitHub repository

```
./target/release/githubtools repo <owner> <repo>
```

2. `activities`: fetch recent activities of a GitHub user

```
./target/release/githubtools activities <user>
```

## License

This tool is licensed under the MIT license.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.
