# autotex Manual

`autotex` is a linux program that runs TeX and LaTeX continuously. For simple example, when

``` bash
autotex testfile.tex
```

runs in bash (or zsh), pdfTeX runs once. This program is written in Rust.

## Installation

It can be installed using `cargo`.

### Using Cargo

``` bash
git clone https://github.com/e0328eric/autotex.git
cd autotex
cargo install --path .
```

## Usage

### Run Plain TeX

There are four options to run this program: `-plain`, `-pdf`, `-xe` and `-lua`. If you run

```bash
autotex -plain testfile.tex
```

in the folder that contains a file to be compiled, then TeX runs and make `testfile.dvi`. Options `-pdf`, `-xe` and `-lua` runs pdfTeX, XeTeX and LuaTeX engine, respectively.

If there is no options like

``` bash
autotex testfile.tex
```

then pdfTeX runs in default.

### Run LaTeX

The option `-la` makes to run LaTeX engines. For example,

``` bash
autotex -la -pdf testfile.tex
```

runs pdfLaTeX and

``` bash
autotex -la -plain testfile.tex
```

runs LaTeX and so on. Like plain TeX case, the only option `-la` makes pdfLaTeX run.

### Continuous Compiling

#### The option `-v`

Many people might want to run TeX engine continuously. This means that TeX engine runs if the compiled file is modified. There is an option what you want. If you run `autotex` like

``` bash
autotex -v testfile.tex
```

then first, TeX engine runs (in this example, there is no engine options, so pdfTeX runs). After TeX engine is finished, pdf viewer is opened and `autotex` waits until the `testfile.tex` file is modified. If this file is modified, then TeX engine runs one more time.

If you want to do this with LuaLaTeX, then run like

``` bash
autotex -v -la -lua testfile.tex
```

as we expected. Note that there is no order for options. So

``` bash
autotex -lua -v -la testfile.tex
```

runs exactly same as in the previous example.

### v0.2.0 upgrade part
#### autotex config
`autotex` can be configurable with yaml file. It must be placed in `~/.config/autotex` and must have name with `config.yaml`.
The example config is in below:

``` reStructuredText
engine:
  main: pdflatex
  latex: pdflatex
pdf: zathura
```
In general, the command `autotex FILENAME` runs pdftex in default. However, if the config file is like in above, it runs pdflatex in default.
Also, `latex` part determines the default latex engine in which `autotex -la FILENAME` runs.
`pdf` part gives the default pdf viewer. The default pdf viewer is `xdg-open`.

