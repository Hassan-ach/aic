# AIC 

An AI-powered CLI tool for command execution through natural language.

## Description

AIC is an intelligent command-line assistant built in Rust that helps users execute commands and automate tasks through natural language interaction. It maintains system safety and enhances user productivity by categorizing commands into different security tiers.

## Features

- **Natural Language Command Execution**: Convert natural language instructions into terminal commands
- **AI-powered Suggestions**: Get intelligent suggestions for command execution
- **Security-focused**: Commands are classified into security tiers to prevent destructive operations
- **Multiple Interaction Modes**:
  - `run`: Execute commands directly
  - `ask`: Send prompts to LLM for assistance
  - `chat`: Interactive chat mode with the AI

## Installation

```bash
# Clone the repository
git clone https://github.com/hassan-ach/aic.git
cd aic

# Build the project
cargo build --release

# The binary will be available at target/release/aic
```

## Usage

```bash
# Run a command directly
aic run "ls -la"

# Ask the AI for assistance
aic ask "How do I find large files in the current directory?"

# Start an interactive chat session
aic chat
```

### Command Options

#### Run Command
```
aic run [COMMAND] [-y] [--verbose]
```
- `-y`: Execute without asking for confirmation
- `--verbose`: Enable verbose mode (default: true)

#### Ask Command
```
aic ask [PROMPT] [-y]
```
- `-y`: Execute without asking for confirmation

#### Global Options
```
-v, --verbose: Enable verbose mode globally
```

## Security

AIC classifies commands into different security tiers:

1. **Tier 1**: Information gathering commands executed immediately
2. **Tier 2**: Safe operations with brief safety notice
3. **Tier 3**: Safe alternatives for potentially risky commands
4. **Tier 4**: Diagnostic alternatives for destructive requests

## Model Configuration

AIC uses the [genai](https://crates.io/crates/genai) crate to interact with AI models. By default, it's configured to use `gemini-2.0-flash` but supports multiple AI providers through environment variables.

### Environment Setup

To use AIC, you need to set the following environment variable:

```bash
# Set your API key for the AI model
export MODEL_API_KEY=your_api_key_here
```

The tool uses Google's Gemini model by default, but you can modify `ai_client.rs` to support other providers like:
- OpenAI (gpt-4o-mini, etc.)
- Anthropic (claude-3-haiku, etc.)
- Cohere (command-light)
- Groq (llama-3.1-8b-instant)
- And more

### Default Model

```rust
pub const MODEL: &str = "gemini-2.0-flash";
```

The authentication system uses a custom resolver that looks for the API key in the `MODEL_API_KEY` environment variable.

## Dependencies

- [clap](https://crates.io/crates/clap): Command-line argument parsing
- [genai](https://crates.io/crates/genai): AI model interaction
- [tokio](https://crates.io/crates/tokio): Asynchronous runtime
- [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json): Serialization/deserialization

## License

[MIT](LICENSE)
