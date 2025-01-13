# Smart Contract Validator

A validator service that listens to smart contract events and executes computational tasks.

## References
- [Twitter Discussion](https://x.com/askwhyharsh/status/1876214220880060702)
- [Excalidraw Documentation](https://t.co/lzj4bW97U1)

## Overview

https://youtu.be/YYPECOIkSMM

This service acts as an operator/validator that monitors smart contract events and performs requested computations. Currently supports JavaScript execution on Anvil networks, with more language support planned.

<img width="608" alt="image" src="https://github.com/user-attachments/assets/86de438c-b37c-4559-bda8-660fec7cf4d5" />

Now, for now the above works

later on, the experience of calling a function, will not require arURI or anything, it would be as easy as maybe 
- calling the name of the function, 
- providing the inputs
- payment for the execution
- number of responses (from validators, how many validotrs should submit the inference)

below is how we can create a registry of commonly used function, 
<img width="854" alt="image" src="https://github.com/user-attachments/assets/6bf6734c-e3f0-4834-a213-a97a53539ddd" />

## Prerequisites

- Rust (for running the validator)
- Node.js (for JavaScript execution)
- Anvil (for local blockchain)

## Getting Started

1. Start an Anvil node:
```bash
anvil --chain-id 31337 --fork-url https://eth.drpc.org -p 3001
```

2. Run the validator:
```bash
cargo run
```

The validator will now listen for computation requests from the smart contract and execute them automatically.

## Current Features

- Event monitoring for computation requests
- JavaScript code execution

## Contributing
I welcome contributions! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow the existing code style
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting PRs


## Contact

twitter: [@askwhyharsh](https://x.com/askwhyharsh)
