![Turing Complete Status](https://img.shields.io/badge/turing%20complete-complete-green?style=for-the-badge)

![Turing Logo](assets/turing_fully_transparent_white_logo.png)

# Turing - A Simple Turing Machine

> It's a [turing machine](https://en.wikipedia.org/wiki/Turing_machine)!

## Installation

### Windows

1. Download the `turing_windows_x86_64.exe` file from the [Releases](https://github.com/Uncodeable864/turing/releases) page.
1. Rename the file to `turing.exe` (case-sensitive)
1. Create a folder in your Desktop called `turing-src` (case-sensitive)
1. Move `turing.exe` to that folder
1. Click the Windows key on your keyboard
1. Type `environment variable`
1. Click "Edit system environment variables". If you're wandering what this does, this will make it so you can just type `turing` into the command line, rather than `some/thing/thing/turing.exe`.
1. Click on "Enviorment Variables"
1. Under "System variables" click "Edit..."
1. Click "Browse"
1. Find the folder where you put the `turing.exe` you downloaded.
1. Click "OK"
1. Click "OK". Close out of the window.
1. See the "Usage" section

### macOS (Apple Silicon)

1. Open Terminal
2. Run the following command:

```bash
curl https://raw.githubusercontent.com/Uncodeable864/turing/main/install/apple_m1.sh | bash
```

### macOS (Intel/x86-64)
1. Open Terminal
2. Run the following command:

```bash
curl https://raw.githubusercontent.com/Uncodeable864/turing/main/install/apple_intel.sh | bash
```
## File type

`*.trng`, short for `turing`

## Language design

To see the language design, please see the file [LANGUAGE_DOC.md](LANGUAGE_DOC.md)
