# Rust Explorer (REX)

<img src="https://github.com/2alf/rex/assets/113948114/7cba4182-5ebd-405a-a134-892cdf5f5c9e" alt="rEx" width="300"/>

## Overview

Rust Explorer (REX) is a simple command-line tool written in Rust that allows users to search for files on their system using a fast and efficient algorithm. The tool provides a user-friendly interface for selecting the drive, entering the file name, and quickly finding the file's location.

## Usage

1. Run the REX executable.
2. Select the drive where you want to search for the file.
3. Enter the name of the file you want to find.
4. Wait for the search to complete.
5. View the results, including file location and search duration.

## Available Platforms

- Windows
- (Planned support for additional platforms)

## Installation

### Windows

1. [Download the latest release](#) of REX for Windows.
2. Run the executable file.

(Instructions for other platforms will be added as support is implemented.)

## Features

- Fast and efficient file search.
- User-friendly interface for drive selection and file searching.
- Displays file location and search duration.

## Example

```bash
$ rex.exe

[REX] Select a drive:
1. C
2. D
Enter the number of the drive: 1

[REX] Enter the file name you want to find: example.txt

[REX] Searching for 'example.txt' on drive C:...

[REX] File found at: C:\Path\to\example.txt
[REX] Search completed successfully.
```

### Future Plans
- Extend platform support.
- Enhance search algorithms for even faster results. Currently O(n).
- Make a GUI.
