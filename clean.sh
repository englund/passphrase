#!/usr/bin/env bash

illegal_chars=-.
hasLength=(4 5)

while IFS=$'\n' read -r line;
do
    # Get the length of the line
    length=${#line}

    if [[ "${hasLength[@]}" =~ $length ]] && [[ $line != *[$illegal_chars]* ]] && [[ $line != [A-ZÅÄÖ]* ]]; then
        echo "$line"
    fi
done