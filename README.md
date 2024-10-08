Flare Rust is a Rust application that interacts with AWS DynamoDB to manage versioning for a service. It allows users to select an AWS profile interactively, read the current version from DynamoDB, and update it if necessary.

## Features

- Interactively select an AWS profile using arrow keys.
- Read the current version from DynamoDB.
- Validate and update the version in DynamoDB.

## Prerequisites

- Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
- AWS credentials configured in your environment or through AWS CLI.
- A DynamoDB table named `flare-platform-staging` with a primary key `Service` and an attribute `Version`.

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/yourusername/flare-rust.git
   cd flare-rust
   ```

2. Install dependencies:

   ```sh
   cargo build
   ```

## Usage

1. Run the application:

   ```sh
   cargo run --release
   ```

2. Follow the prompts to select an AWS profile and enter a new version.

## Building the Release Binary

To build the release version of your Rust program:

```sh
cargo build --release
```

The optimized binary will be located in the `target/release` directory. You can find your executable at `target/release/flare-rust`.

Example

```sh
$ cargo run --release
Select an AWS profile:
> default
  profile1
  profile2
Current version: 1.0.0
Please enter a new version: 1.1.0
Update successful!
New version: 1.1.0
```

Dependencies
- aws-config
- aws-sdk-dynamodb
- aws-types
- aws-runtime
- colored
- inquire
- semver
- tokio
License
This project is licensed under the MIT License.
