# Rust OpenAI Chatbot CLI

![Rust Version](https://img.shields.io/badge/Rust-1.55+-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

This is a command-line interface (CLI) program built using Rust that integrates with OpenAI's chatbot using the OpenAI API. It allows users to have interactive conversations with the chatbot directly from their terminal.

## Features

- Interactive conversations with the OpenAI chatbot.
- Seamless integration with the OpenAI API.
- Customizable conversation prompts and options.
- Simple and user-friendly interface.

## Prerequisites

- Rust 1.55+ installed. If not, you can download it from [here](https://www.rust-lang.org/tools/install).
- OpenAI API key. You can obtain it by signing up at [OpenAI](https://beta.openai.com/signup/).

## Installation

1. Clone this repository to your local machine:

   ```sh
   git clone https://github.com/elwin212/rust-cli-openai.git
   cd rust-openai-chatbot-cli
   ```
2. Set your OpenAI API key as an environment variable:

   ```sh
   export OPENAI_API_KEY=your_api_key_here
   ```
3. Build and run the CLI program:
  ```sh
  cargo build --release
  cargo run
  ```

## Configuration
You can customize the behavior of the chatbot by modifying the configuration options in the src/main.rs file.

## License
This project is licensed under the MIT License.
