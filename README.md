# COD

Counts lines of code (Similar to [cloc](https://github.com/AlDanial/cloc)
but adds some additional information like number of functions and variables.
It can also print information more specific to a certain programming language
like the number of templates in C++ files.

## Usage

Run the binary with . as an arguments and it will recursively traverse the current directory
(respecting .gitignore) and print data about the files it finds. To only consider specific
files/directories they can be passed as arguments or -i (repeatedly) can be used to ignore
certain files or directories.

For more detailed information about a language the -l option can be used.

## Example
```
$ cod . -l cpp
Language       Files          Total lines    Blank lines    Functions      Variables      Loops
=========================================================================================================
C              1              22             6              2              4              3
Cpp            1              29             8              2              4              4
Markdown       1              17             5              0              0              0
Other          2              21             4              0              0              0
Rust           5              575            46             26             48             18
Toml           1              17             2              0              0              0
Zig            1              27             5              2              9              4
---------------------------------------------------------------------------------------------------------
Total          12             708            76             32             65             29

*** Cpp ***
Number of files: 1
Total lines: 29
Blank lines: 8
Variables: 4
Templates: 1
Functions: 2
Defines: 1
Loops: 4
```
