# pacify
Pacify is a Yet another simple Python build/package manager(Wrapper) designed to streamline the process of managing Python projects. With a straightforward command-line interface, Pacify aims to make Python project management a breeze.

## Features

- Initialize a new project
- Create a new project
- Run a project
- Clean a project
- Add a dependency with optional version specification
- Update a package (WIP)
- Remove a package (WIP)

## Installation

Currently, Pacify is in its early stages, and installation instructions will be provided as development progresses.

## Usage

### Initialize a New Project

```bash
pacify init [path]
```

- `path`: The path where the project should be initialized. Defaults to the current directory.

### Create a New Project

```bash
pacify new
```

### Run a Project

```bash
pacify run
```

### Clean a Project

```bash
pacify clean
```

### Add a Dependency

```bash
pacify add <package_name> [-v <version>]
```

- `package_name`: The name of the package you want to add.
- `-v, --version`: (Optional) The version of the package you want to add.

### Update a Package (WIP)

```bash
pacify update <package_name>
```

- `package_name`: The name of the package you want to update.

### Remove a Package (WIP)

```bash
pacify remove <package_name>
```

- `package_name`: The name of the package you want to update.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on our GitHub repository.

## License

Pacify is open-source software. The license details will be provided as development progresses.

## Contact

For any inquiries, suggestions, or feedback, please reach out to:

**Skuld Norniern**  
Email: skuldnorniern@gmail.com

---

Note: This README is based on the provided `main.rs` file. Some features might be under development, and the README will be updated accordingly as the project evolves.
