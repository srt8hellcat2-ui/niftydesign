# Restore the test data

The pinned inputs are 142 controlled publications, 24 real-file test publications, 142 scoring JSON files and nine reference PNGs: 317 files totaling 17,482,183 bytes (16.67 MiB). The [manifest](../scripts/niftydata.manifest) records each relative path, byte size, SHA-256 and full pinned source URL. Its binary rows come from the existing fixture CSV; scoring and image rows come from [the source record](pub-format/SOURCES.md). Each entry was checked against the existing verified data; no hash was invented.

Keep these files outside the checkout. They are test inputs and scoring references, not product assets. Scoring JSON and reference images may score output but must never supply a storage interpretation. Restoring downloads only the listed data files, never an upstream checkout, implementation or extra sample.

The durable location is `/workspace/niftydata`. The familiar `/tmp/niftydata` path is a symlink to it. Workspace storage avoids ordinary temporary-directory cleanup; a replacement machine still needs to restore missing data from the pinned sources.

Run this on a new machine after checking out the repository:

```sh
/workspace/niftydesign/scripts/restore-niftydata.sh
```

The first run downloads missing files. Later runs verify size and SHA-256 and skip correct files. Wrong files stay in place until a replacement download has passed both checks; replacement is atomic. Downloads have connection and total timeouts and three retries. Only pinned `raw.githubusercontent.com` URLs are accepted, using secure connections without following redirects. The restore needs no repository listing endpoint or unrestricted network access.

Each repaired problem prints its relative file path and original problem. Each failure prints the exact file and error, and the script exits nonzero if any input cannot be restored. A failed download or bad replacement never becomes a valid cached input. Rerun the same command after access is restored; already verified files are reused. Do not treat a partial restore as a valid corpus.

An existing real file or directory at `/tmp/niftydata`, or a symlink pointing elsewhere, is left unchanged and reported as an error. To prepare the durable store while preserving an old real directory, use `--no-link`, keep the old directory until both copies are verified, and move it aside yourself before rerunning the normal command. The script never removes or moves the old corpus for you.

For a separate scratch restore without touching the normal alias:

```sh
./scripts/restore-niftydata.sh --dest /tmp/restore-check/data --link-path /tmp/restore-check/alias
```

Create the parent of the requested alias first. Alternatively use `--no-link`. Data destinations inside the checkout are refused. Extra files, including previous generated runs, are left alone.

To use a restored copy for the existing corpus gates, point both variables at that copy:

```sh
export PUB_CORPUS=/workspace/niftydata/pubsmith-fixtures
export PUB_REAL_CORPUS=/workspace/niftydata/apache-pub
```

## Generated reports and snapshots

These are derived outputs, not additional downloads or manifest inputs. With the existing development shell active, optionally run:

```sh
./scripts/restore-niftydata.sh --rebuild-generated
```

This first verifies all inputs, then runs exactly the two generation commands recorded in [the pixel recipe](pub-format/pixel-diff.md#run): reference-scoring mode followed by snapshot mode. Reference-scoring mode rebuilds `pixel-report/scores.csv`, `report.md` and nine side-by-side images; it replaces the previous report. Snapshot mode renders all 142 controlled first pages and writes a complete manifest under a new `pixel-snapshots/restore-<time>-<process>` directory. Use `--snapshot-run NAME` for a chosen name. Existing snapshot runs are never overwritten; failed runs can leave a partial directory and require a new run name.

The existing generation commands write to `/tmp/niftydata`. The optional step refuses to run unless that path resolves to the chosen destination. It does not redirect the runner, change product code, install dependencies or change shell settings. Scratch restores with a different alias can test input restoration but cannot redirect these generated outputs. The same recipe's comparison mode compares two complete snapshot runs; a single regenerated run is not a before/after comparison.

UNKNOWN: original historical images, timestamps and manifests from a lost machine cannot be reconstructed byte for byte from these steps. Current outputs can be regenerated, but fonts, platform and current code affect their pixels. These snapshots are our own output, not additional reference images.

## Timing, space and verification

Input contents use 17,482,183 bytes; allocated disk use is slightly higher because of filesystem blocks. Restoration temporarily needs room for one extra input file. Generated outputs and build products require additional space. Duration depends on the network, cache and any compilation; the first empty scratch restore took 108.497 seconds in this session. Verified-repeat and repair timings are still being measured.

The first validation run restored all 317 files from empty to a fresh scratch folder and created its separate alias successfully. Corrupt/deleted-file repair, final gate counts and generated-output checks are still being collected. This note and the restore files do not change the environment install or activation scripts. Installing the future hook remains a separate owner action; if setup cannot fetch inputs, it must warn and leave the development installation usable. Creating an alias in a later shell does not populate missing data.
