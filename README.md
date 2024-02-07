# Version Command

This command selects a program version.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *version* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/version/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/version/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *version* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/version/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *version*.

## Usage

To select version for given program, type:

```bash
$ version program
```

For instance, to choose GO version, you would type:

```bash
$ version go
Please choose a version:
0: System
1: 1.21.7
1
```

This will print a menu with available versions for given program. You can then select a version by typing its number. If you select *0*, the system version will be used. If you select a version, *version* will create a symbolic link in */opt/software* from selected version to *current*.

*Enjoy!*
