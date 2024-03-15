# Contributing to lprs

Thank you for your interest in contributing to lprs! We welcome contributions from the community to help improve the project.

## Reporting Issues

If you encounter any issues or bugs while using lprs, please open a new issue on the Forgejo repository. When reporting an issue, please provide as much detail as possible, including steps to reproduce the issue and any relevant error messages.

## Feature Requests

If you have a feature request or an idea for improving lprs, we encourage you to open a new issue on the Forgejo repository. Please describe the feature or improvement in detail and provide any relevant context or examples.

## Writing Code
Before you start writing code, please open a new issue first to discuss the proposed changes. This will help ensure that your contribution is aligned with the project's goals and that you are not duplicating work that is already in progress or has been completed by someone else.

### PR title
Your PR will squash and merge, and your PR title will be used as the commit message. Please make sure your PR title is clear and concise.

The title must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format. This means that the title should be in the following format:

```
<type>(<scope>): <description>
```

- The `<scope>` is optional, and the `<description>` should be a clear and concise summary of the changes.
- You should use the imperative, present tense (e.g., "Add feature" instead of "Added feature").
- The `<type>` should be one of the following:
  - `feat`: A new feature
  - `fix`: A bug fix
  - `docs`: Documentation changes
  - `refactor`: Refactoring code without changing its behavior
  - `change`: Changes that affect the meaning of the code
  - `deprecate`: Changes that deprecate a part of the code
  - `remove`: Changes that remove a deprecated part of the code
  - `security`: Changes that affect the security of the code
  - `perf`: A code change that improves performance
  - `test`: Adding missing tests or correcting existing tests
  - `chore`: Changes to the build process or auxiliary tools and libraries such as documentation generation

#### Example
```
- feat: something
- chore(ci): update MSRV
```

### PR description
Your PR description should provide a clear and concise summary of the changes you have made. It should also include any relevant context or background information that will help the project maintainers understand the purpose of the changes. Make sure to reference the issue that your PR is addressing, and note any breaking changes that your PR introduces.

Make sure to explain why you made the changes not just what changes you made.

### Code Style

Please follow the existing code style and conventions used in the lprs project. This includes:

- Using Rust's official formatting tool, `rustfmt`, to format your code.
- Writing clear and concise code with meaningful variable and function names.
- Adding comments to explain complex logic or algorithms.

### CI
Run the CI before submitting your code. You can run the CI with the following command:

```bash
just ci
```

This will run the tests and check the code formatting. If the CI fail, please fix the issues before submitting your code.

## Code Review

All contributions to lprs will go through a code review process. This ensures that the code meets the project's standards and maintains its quality. Please be open to feedback and suggestions from the project maintainers during the code review process.

## License

By contributing to lprs, you agree that your contributions will be licensed under the project's [LICENSE](LICENSE) file. This means that you are granting lprs the right to use, modify, and distribute your contributions under the terms of the license. wich is GPL-3.0 License.

Happy contributing!
