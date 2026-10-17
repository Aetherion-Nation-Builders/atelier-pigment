#!/usr/bin/env bash
# Atelier: turn a test log into job annotations (the failing tests, panics and errors), so they
# can be read through the API by the build agents, whose network does not reach the job log
# store. Usage: scripts/ci-report-failures.sh test.log. Always exits 0.
set +e
log="${1:-test.log}"
[ -f "$log" ] || { echo "::error title=Test log::$log missing (the step failed before testing)"; exit 0; }
# CARGO_TERM_COLOR=always: strip the colour codes before matching.
sed 's/\x1b\[[0-9;]*[A-Za-z]//g' "$log" > "$log.plain"
{
  grep -E "^test .* FAILED$" "$log.plain" | sed 's/^test //; s/ \.\.\. FAILED$//'
  grep -E "^ +(FAIL|SIGSEGV|SIGABRT|TIMEOUT) \[" "$log.plain" | sed -E 's/^ +[A-Z]+ \[[^]]*\] +//'
} | sort -u | head -15 | while read -r name; do
  echo "::error title=Failing test::${name}"
done
grep -E "panicked at|^error(\[E[0-9]+\])?:|No space left|SIGKILL|signal: " "$log.plain" | sort -u | head -10 | while read -r line; do
  echo "::error title=Test output::${line:0:900}"
done
echo "::error title=Test log tail::$(tail -c 1500 "$log.plain" | tr '\n' ' ' | cut -c1-1400)"
exit 0
