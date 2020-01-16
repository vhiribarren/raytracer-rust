#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PUBLISH_DIR="gh-pages"

cd $SCRIPT_DIR && cd ..
npm run clean
npm run build
mkdir -p $PUBLISH_DIR && cd $PUBLISH_DIR
git clone git@github.com:vhiribarren/raytracer-rust.git --depth 1 -b gh-pages .
rm *
cp ../dist/* .
git add .
git commit --amend --no-edit
git push -f