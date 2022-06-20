# Emu6502
A simple 6502 machine code styled interpreter. I might turn it into it's own scripting language in the future, who knows.
## Program setup
Make a .sfot file with a size of 0x10000 (65536) bytes, write your code to it with a hex editor of some kind, and you've got a script. This is essentially just a map of memory that the interpreter takes in as input. You'll want to set the word at 0xFFFC to where in memory your program starts, keep in mind that ROM starts at E000.
## Usage
Once the program is built, simply run is as `.\emu6502.exe .\yourprogramename.sfot`. This will vary on unix based platforms, but if you're on linux you can figure it out.