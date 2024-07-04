#!/bin/bash

FILES=$(ls -d ./contracts/*)

for FILE in $FILES; do
    $(
        cd ./contracts/$FILE
        cargo schema
    )
done

FILES=$(ls -d ./packages/*)

for FILE in $FILES; do
    $(
        cd ./packages/$FILE
        cargo schema
    )
done
