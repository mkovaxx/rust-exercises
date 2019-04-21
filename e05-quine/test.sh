#!/usr/bin/env bash

OUTPUT=$(cargo -q run)
SOURCE=$(cat src/main.rs)

if [ "$OUTPUT" = "$SOURCE" ]; then
    tput setaf 2 && tput bold && echo -n "[SUCCESS]" && tput sgr0
    echo " Output matches source code."
    exit 0
else
    tput setaf 1 && tput bold && echo -n "[FAILURE]" && tput sgr0
    echo " Output and source code differ:"
    diff --side-by-side <(echo "$OUTPUT") <(echo "$SOURCE")
    exit 1
fi;
