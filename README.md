# Envoke

Envoke is a simple, extremely light command-line interface (CLI) tool written in
Rust for managing multiple environment profiles. It allows users to initialize a
directory, create, switch, list, and remove profiles, and check the currently
active profile. Envoke is designed to be simple, fast, and reliable for developers
who need to manage environment configurations efficiently.

## Features

- **Initialize a Directory**: Set up a directory for managing profiles with the `init` command.
- **Create Profiles**: Create new environment profiles using the `create` command.
- **Switch Profiles**: Seamlessly switch between profiles with the `switch` command.
- **List Profiles**: View all available profiles with the `list` command.
- **Remove Profiles**: Delete profiles permanently using the `remove` command.
- **Check Current Profile**: Display the currently active profile with the `current` command.

## Profile Management

Envoke follows a **convention over configuration** approach. When you run the
`init` command, a `.envoke` directory is created in the current working directory.

A profile is simply any `<profile>.env` file located in the `.envoke` directory.

- **Profile Creation**: When you create a profile with `envoke create <PROFILE>`, Envoke stores the `<profile>.env` file within the `.envoke` directory.
- **Symlinking**: When you switch to a profile using `envoke switch <PROFILE>`, Envoke creates a symbolic link (symlink) to the corresponding `.env` file for that profile.
- **Profile Deletion**: When you remove a profile with `envoke remove <PROFILE>`, the corresponding `<profile>.env` file is permanently deleted. If that profile was the currently active profile, the symlink will also be removed.

This approach ensures that environment configurations are cleanly managed within
the `.envoke` directory, with the active `.env` file always reflecting the current profile.

## System Requirements

Envoke is designed to work on Unix-like systems (e.g., Linux, macOS). It will not
function on non-Unix-like systems.

## Installation

To install Envoke, you need to have Rust installed: https://www.rust-lang.org/

Then, clone this repository and build the project:

```bash
git clone https://github.com/TSxo/envoke.git
cd envoke
cargo build --release
```

The compiled binary will be available in the `target/release` directory. You can move it to a
directory in your `$PATH` for global access:

```bash
sudo mv target/release/envoke /usr/local/bin/
```

## Commands

```
Usage: envoke <COMMAND>

Commands:
  init     Initializes the directory
  create   Creates a new profile
  switch   Switch to a specified profile
  remove   Deletes a profile - cannot be undone
  list     Lists available profiles
  current  Display the current active profile
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Example Workflow

1. Initialize a directory:

   ```bash
   envoke init
   ```

2. Create a development profile:

   ```bash
   envoke create dev
   ```

3. Switch to the `dev` profile:

   ```bash
   envoke switch dev
   ```

4. Check the current profile:

   ```bash
   envoke current
   # Output: dev
   ```

5. Create and switch to a production profile:

   ```bash
   envoke create prod
   envoke switch prod
   ```

6. List all profiles:

   ```bash
   envoke list
   # Output: dev, prod
   ```

7. Remove the `prod` profile:

   ```bash
   envoke remove prod
   ```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgments

Built with Rust and the clap crate for CLI parsing.
