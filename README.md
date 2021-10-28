**I will changed entire code into golang because I want to get a project written in go**

# autotex Manual

`autotex` is a linux program that runs TeX and LaTeX continuously. For simple example, when

```bash
autotex testfile.tex
```

runs in bash (or zsh), pdfTeX runs once. This program is written in Rust.

## Installation

It can be installed using `cargo`.

### Using Cargo

```bash
git clone https://github.com/e0328eric/autotex.git
cd autotex
cargo install --path .
```

## Usage

### Run TeX Engines

Use `--engine` or `-e` option to specify TeX engines. For example, if you want to use plain TeX, type either

```bash
autotex -e tex testfile.tex
autotex --engine tex testfile.tex
autotex -e plaintex testfile.tex
autotex --engine plaintex testfile.tex
```

in the folder that contains a file to be compiled, then TeX runs and make `testfile.dvi`.

Option `--engine` can get `pdftex`, `xetex`, `luatex`, `tex`, `plaintex`, `pdflatex`, `xelatex`, `lualatex` and `latex`.

Another ways to use these options is use specific engine options.
Below table shows flags for each engines.

| TeX Related Engine | Flag  |
| :----------------: | :---: |
|      `pdftex`      | `-p`  |
|      `xetex`       | `-x`  |
|      `luatex`      | `-l`  |
|       `tex`        | `-t`  |
|     `pdflatex`     | `-pL` |
|     `xelatex`      | `-xL` |
|     `lualatex`     | `-lL` |
|      `latex`       | `-L`  |

For example, below arguments are identical:

```bash
autotex --engine xelatex testfile.tex
autotex -e xelatex testfile.tex
autotex -xL testfile.tex
autotex -Lx testfile.tex
```

If there is no options like

```bash
autotex testfile.tex
```

then pdfTeX runs in default.

### Continuous Compiling

#### The option `-c`

Many people might want to run TeX engine continuously. This means that TeX engine runs if the compiled file is modified. There is an option what you want. If you run `autotex` like

```bash
autotex -c testfile.tex
```

then TeX engine runs (in this example, there is no engine options, so pdfTeX runs) and autotex`waits until the`testfile.tex` file is modified. If this file is modified, then TeX engine runs one more time.

If you want to do this with LuaLaTeX, then run like either

```bash
autotex -e lualatex -c testfile.tex
```

as we expected. Note that there is no order for options. So

```bash
autotex -c -e lualatex testfile.tex
autotex -ce lualatex testfile.tex
autotex -ec lualatex testfile.tex
autotex -lLc testfile.tex
autotex -clL testfile.tex
```

runs exactly same as in the previous example.

### View Pdf

If the option `-v` is enabeled, then open a pdf viewer so that we can view the pdf that
compiled from TeX file. Hence, the command, for instance,

```bash
autotex -vcpL testfile.tex
```

means that compile with `pdflatex` continuously and view the pdf file of it.

### v0.2.0 upgrade part

#### autotex config

`autotex` can be configurable with yaml file. It must be placed in `~/.config/autotex` and must have name with `config.yaml`.
The example config is in below:

```reStructuredText
engine:
  main: pdflatex
pdf: zathura
```

In general, the command `autotex FILENAME` runs pdftex in default. However, if the config file is like in above, it runs pdflatex in default.
`pdf` part gives the default pdf viewer. The default pdf viewer is `xdg-open`.
