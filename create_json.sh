#!/bin/bash

PARAMS="--arg @type"
ITER=0
for i in "$@";
do
    if ! ((ITER % 2)); then
        PARAMS="$PARAMS $i"
    else
        PARAMS="$PARAMS --arg $i"
    fi
    (( ITER++ ))
done

jq -n -c $PARAMS '$ARGS.named' | tee /dev/stderr | pbcopy
