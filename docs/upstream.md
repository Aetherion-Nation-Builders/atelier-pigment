# Pulling in upstream work

Upstream is [storytold/photocraft](https://github.com/storytold/photocraft). We merge its `main`
into ours; our changes stay a thin layer on top (`docs/brand.md`).

```bash
scripts/sync-upstream.sh            # fetch upstream, merge into a branch, run the checks
```

The script adds the `upstream` remote if it is missing, creates `sync/upstream-<date>` from
`main`, merges `upstream/main`, and runs `cargo xtask ci` (or the test suite). Open a pull
request from that branch; `main` is protected and is only changed through pull requests.

Conflicts come from the files the brand layer touches. `NOTICE` keeps our version
automatically (`.gitattributes` marks it `merge=ours`; the script enables that merge driver).
`README.md` is our header on top of upstream's feature sections, so upstream README edits merge
in below the line; if the header conflicts, keep ours. For anything else, keep upstream's logic
and re-apply the rename.

CI here runs on Linux only: a cold Windows runner takes over 90 minutes and bills double, and
upstream's own CI covers Windows for the code every sync brings in. `ci.yml`'s platform step
lists the runners; add `windows-latest` back when preparing a Windows release.

Workflows that are turned off in this repository, on purpose (they cost Actions minutes and
build things we do not ship yet): `perf-nightly.yml`, `freebsd.yml`, `windows-arm64.yml`,
`release.yml`. Turn one on from the Actions tab when it is wanted.
