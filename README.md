# mng_raw_cli

`mng_raw_cli` is a command-line tool to manage and organize JPG and RAF photo files. It allows you to:

1. **Organize JPG and RAF files into separate folders**
2. **Move RAF files corresponding to JPGs in `jpg/selected` to `raf/selected`**

## Features

- Automatically creates necessary directories (`jpg`, `raf`, `jpg/selected`, `raf/selected`).
- Moves files to appropriate folders based on their extensions.
- Matches and moves corresponding RAF files when JPGs are placed in `jpg/selected`.

---

## Installation

### Option 1: Using PyInstaller to Build an Executable

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/mng_raw_cli.git
   cd mng_raw_cli
   ```

2. Install PyInstaller if you don't have it:
   ```bash
   pip install pyinstaller
   ```

3. Create an executable:
   ```bash
   pyinstaller --onefile --name mng_raw_cli script.py
   ```

   This will generate an executable in the `dist/` folder named `mng_raw_cli` (or `mng_raw_cli.exe` on Windows).

4. Move the executable to a folder in your `PATH` for easy access:
   ```bash
   sudo mv dist/mng_raw_cli /usr/local/bin
   ```

Now you can run the tool directly from the terminal.

### Option 2: Pre-built Binary

If a pre-built binary is available (e.g., in GitHub Releases):

1. Download the binary for your operating system.
2. Move the binary to a folder in your `PATH` (e.g., `/usr/local/bin`):
   ```bash
   sudo mv mng_raw_cli /usr/local/bin
   ```

---

## Usage

Run the tool with one of the following options:

### Divide Photos
Organize JPG and RAF files into respective folders:
```bash
mng_raw_cli -divide /path/to/photos
```

- Moves `.JPG` files to the `jpg` folder.
- Moves `.RAF` files to the `raf` folder.
- Leaves other file types in the root directory.
- Creates `jpg/selected` and `raf/selected` directories if they don't exist.

### Select RAF from JPG
Move RAF files corresponding to JPG files in `jpg/selected` to `raf/selected`:
```bash
mng_raw_cli -select /path/to/photos
```

- Matches `.RAF` files to `.JPG` files in `jpg/selected`.
- Moves matching RAF files to `raf/selected`.
- Skips RAF files that are already in `raf/selected`.

---

## Example

### Input Directory Structure
```
test/
├── photo1.JPG
├── photo1.RAF
├── some_other_file.ext
```

### After Running `mng_raw_cli -divide test`
```
test/
├── some_other_file.ext
├── jpg/
│   ├── photo1.JPG
│   └── selected/
├── raf/
│   ├── photo1.RAF
│   └── selected/
```

### After Adding `photo1.JPG` to `jpg/selected` and Running `mng_raw_cli -select test`
```
test/
├── some_other_file.ext
├── jpg/
│   ├── photo1.JPG
│   └── selected/
│       └── photo1.JPG
├── raf/
│   ├── selected/
│       └── photo1.RAF
```

---

## Development

Feel free to contribute to this project by submitting issues or pull requests. To set up a development environment:

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/mng_raw_cli.git
   cd mng_raw_cli
   ```

2. Install dependencies
   ```bash
   uv sync
   ```   

3. Test the functionality by running the script directly:
   ```bash
   python script.py -divide /path/to/photos
   python script.py -select /path/to/photos
   ```

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Notes

- PyInstaller-generated executables are platform-specific. To distribute binaries for multiple platforms, you need to run `pyinstaller` on each target platform or use CI/CD tools like GitHub Actions.
- Ensure the folder paths provided as arguments are accessible and contain the necessary files.
