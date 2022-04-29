#!/usr/bin/env bash

expected=$(cargo run -q)

while true; do
    actual=$(cargo run -q)
    if [ "$expected" != "$actual" ]; then
        expected=$(cargo run -q)
        echo "
----- CHANGE OCCURED $(date) -----

$actual

" | tee -a changes.txt


    fi

sleep 3
done

