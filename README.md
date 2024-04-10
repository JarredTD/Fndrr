# Fndrr

Fndrr is a simple script to find any files within a directory and its subdirectories that contain the given text as part of its file name.

## Getting Started

### Installing

- Go to the latest release and download the binary

### Building

1. Clone the project
2. Compile it with Cargo: `cargo build --release`
3. Find the executable under `target/release/`

### Executing program

```bash
./fndrr <file_name> [start_path]
```

- `start_path` by default is the current directory the script is called in
- Run `./fndrr -h` for help

## Contributing

### Commit Messages

Use the conventional commit format for all your commits. This helps in automating our release process and maintaining a clear history. A conventional commit message should look like this:

```markdown
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types include:

```markdown
- feat: Introduces a new feature to the project.
- fix: Fixes a bug in the project.
- docs: Changes to documentation only.
- style: Code changes that do not affect the meaning (white-space, formatting, missing semi-colons, etc).
- refactor: Code changes that neither fix a bug nor add a feature.
- perf: Changes that improve performance.
- test: Adding missing tests or correcting existing tests.
- build: Updates to the build process.
- chore: Changes to auxiliary tools and libraries such as documentation generation.
```

For more details, refer to the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the [LICENSE](LICENSE) file for details. AGPL-3.0 is a free, copyleft license designed specifically for software and other kinds of works, offering the freedom to run, study, share, and modify the software. It's similar to the GPL-3.0 but with an additional term to cover the use of the software over a network.

For more details on the AGPL-3.0 License, please refer to [gnu.org/licenses/agpl-3.0](https://www.gnu.org/licenses/agpl-3.0).
