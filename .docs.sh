#!/bin/bash
set -ev

if [ "$TRAVIS_PULL_REQUEST" == "false" ] && [ "$TRAVIS_BRANCH" == "master" ]; then
    echo -e "Building docs...\n"
    cargo doc --no-deps

    echo -e "Publishing docs...\n"
    git clone --quiet --branch=gh-pages https://${GH_TOKEN}@github.com/b52/angular-rust gh-pages > /dev/null
    cp -rf target/doc/* gh-pages/
    cd gh-pages
    git config user.email "travis@travis-ci.org"
    git config user.name "travis-ci"
    git add -f .
    git commit -m "Lastest docs of successful travis build $TRAVIS_BUILD_NUMBER auto-pushed to gh-pages"
    git push -fq origin gh-pages > /dev/null

    echo -e "Published docs to gh-pages.\n"
fi
