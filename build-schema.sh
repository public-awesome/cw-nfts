#!/bin/bash

FILES=$(ls -d ./contracts/*)

for FILE in $FILES; do
    $(
        cd $FILE
        cargo schema
    )
done

FILES=$(ls -d ./packages/*)

for FILE in $FILES; do
    echo "================= $FILE ================="
    $(
        cd $FILE
        cargo schema
    )
done
