# Smart Contract Validator

A validator service that listens to smart contract events and executes computational tasks.

## References
- [Twitter Discussion](https://x.com/askwhyharsh/status/1876214220880060702)
- [Excalidraw Documentation](https://t.co/lzj4bW97U1)

## Overview

This service acts as an operator/validator that monitors smart contract events and performs requested computations. Currently supports JavaScript execution on Anvil networks, with more language support planned.

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

## Roadmap
- [ ] Performance optimizations and more details
- [ ] Support for additional programming languages
- [ ] Proof of execution in some way
- [ ] Enhanced error handling

## Contributing

We welcome contributions! Here's how you can help:

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