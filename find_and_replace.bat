echo on

pause

cd "C:\Rust\find_and_replace"

pause

rustc find_and_replace.rs

pause

find_and_replace.exe "sii" "C:\Mods\ATS\Work\Parts\def\vehicle\truck\common\engine\factory" "" "torque:"

pause

del find_and_replace.exe
del find_and_replace.pdb