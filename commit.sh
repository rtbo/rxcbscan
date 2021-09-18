#! /bin/bash

mkdir -p ../rust-xcb/build/codegen

for f in ../rust-xcb/build/*.rs; do
    rm $f
done
for f in ../rust-xcb/build/codegen/*.rs; do
    rm $f
done

for f in src/*.rs; do
    cp $f ../rust-xcb/build
done
for f in src/codegen/*.rs; do
    cp $f ../rust-xcb/build/codegen
done

MSG="$1"

if [ -n "$MSG" ]; then
    git add -A src/*
    git commit -m "$MSG"

    pushd ../rust-xcb
        git add -A build/*
        git commit -m "$MSG"
    popd
fi
