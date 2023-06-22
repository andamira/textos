#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

cmd="cargo c"; echo "std, safe\n$ " $cmd; $cmd
cmd="cargo cu"; echo "std, unsafe\n$" $cmd; $cmd
cmd="cargo cn"; echo "no-std, safe\n$" $cmd; $cmd
cmd="cargo cNu"; echo "no-std, no-alloc, unsafe\n$" $cmd; $cmd

cmd="cargo t"; echo "tests\n$" $cmd; $cmd
cmd="cargo tu"; echo "tests\n$" $cmd; $cmd

cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd
