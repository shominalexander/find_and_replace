echo on

cd "C:\Rust\find_and_replace"

rustc source.rs

pause

source.exe "sii" "C:\Mods\ATS\Work\Parts\def\vehicle\truck\common\engine\handmade\tuning" "" "" "" "price" "0" "" ""

pause

del source.exe
del source.pdb

pause
