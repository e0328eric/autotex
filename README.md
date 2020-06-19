# autotex Manual

`autotex` is a linux program that runs TeX and LaTeX continuously. For simple example, when

``` bash
autotex testfile.tex
```

runs in bash (or zsh), pdfTeX runs once. This program is written in Haskell.

## Installation

It can be installed using `cabal` or `stack`.

### Using Cabal

``` bash
git clone https://github.com/e0328eric/autotex.git
cd autotex
cabal install autotex
```

### Using Stack

``` bash
git clone https://github.com/e0328eric/autotex.git
cd autotex
stack install
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

#### PDF viewer setting

The default setting of `autotex` is `xdg-open`. However, you can set this with other pdf viewer, for example, `qpdfview` which I use it.

To do this, make a file `.autotexrc` on your home direcrory and write `.autotexrc` like this:

``` reStructuredText
pdfview : qpdfview
```

### Compile TeX in Nasted Folder

For example, I want to make a Folland real analysis solution with pdfTeX, so I make a directory like this:

```text
Folland  |--- folland.tex
		 |--- Chapter  |--- ch1.tex
		 	       |--- ch2.tex
		 	       |--- ch3.tex
		 	       |--- ch4.tex
		 	       |--- ch5.tex
```

I should compile `folland.tex` to make a solution. But actually I want to run `autotex` in the `Chapter` folder. In this case, run autotex like

``` bash
autotex -cd ../folland.tex
```

if I want to compile once and

``` bash
autotex -cd -v ../folland.tex
```

if I want to compile continuously.
