#!/bin/bash

gcc -o komendy commands.c
gcc -o ko copy.c
gcc -o a smbdScript.c

echo "Kompilacja zakonczona. Pliki wykonywalne: komendy, ko, a"
