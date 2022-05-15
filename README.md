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

### macOS

1. Download the file from the [Releases](https://github.com/Uncodeable864/turing/releases). If you use Apple Silicon download `turing_apple_arm64`, otherwise download `turing_apple_x86_64_darwin`.
1. Rename the file you download to `turing` (case-sensitive), no file extension
1. Create a folder in your Desktop called `turing-src`. Move `turing` there.
1. In your Terminal run: `sudo +x ~/Desktop/turing-src/turing`
1. Next run: `echo `

## File type

`*.trng`, short for `turing`

## Language design

To see the language design, please see the file [LANGUAGE_DOC.md](LANGUAGE_DOC.md)
