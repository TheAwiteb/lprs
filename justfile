# This justfile is for the contrbutors of this project, not for the end user.
#
# Requirements for this justfile:
# - Linux distribution
# - just (Of course) <https://github.com/casey/just>
# - cargo (For the build and tests) <https://doc.rust-lang.org/cargo/getting-started/installation.html>

set quiet
set shell := ["/usr/bin/env", "bash", "-c"]

JUST_EXECUTABLE := "just -u -f " + justfile()
header := "Available tasks:\n"

_default:
    @{{JUST_EXECUTABLE}} --list-heading "{{header}}" --list

# Run the CI
@ci: && msrv
    cargo build -q
    cargo fmt -- --check
    cargo clippy -- -D warnings

# Check that the current MSRV is correct
@msrv:
    cargo-msrv verify

# Deploy the book to Github Pages
@deploy:
    #!/usr/bin/env bash
    mdbook build
    cd book
    git init .
    git checkout -B gh-pages
    touch .nojekyll
    echo "lprs.4rs.nl" > CNAME

    git add .
    git commit -m "Deploy the book to github pages"
    git remote add origin "https://github.com/TheAwiteb/lprs-book"
    git push origin gh-pages -f
    cd ..
    rm -fr book
