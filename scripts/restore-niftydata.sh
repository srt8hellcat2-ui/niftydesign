#!/usr/bin/env bash
# Copyright (c) 2026 srt8hellcat2-ui. SPDX-License-Identifier: MIT OR Apache-2.0
# Restore pinned data only; never fetch an upstream checkout or reader source.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT/scripts/niftydata.manifest"
dest=/workspace/niftydata
link_path=/tmp/niftydata
make_link=1
rebuild=0
snapshot_run="restore-$(date -u +%Y%m%dT%H%M%SZ)-$$"
problem() { printf 'ERROR %s: %s\n' "$1" "${2//$'\n'/; }" >&2; }
usage() {
  cat <<'HELP'
Usage: restore-niftydata.sh [--dest DIR] [--no-link | --link-path PATH]
                           [--rebuild-generated] [--snapshot-run NAME]
Defaults: restore /workspace/niftydata, then link /tmp/niftydata to it.
--dest DIR            Separate destination for testing; never inside the checkout.
--no-link             Leave /tmp/niftydata untouched (scratch restores).
--link-path PATH      Use a separate link path for testing.
--rebuild-generated   Rebuild current pixel report and a new first-page snapshot.
--snapshot-run NAME   Name the new snapshot run; existing runs are never replaced.
The generated runner requires /tmp/niftydata to resolve to the destination.
HELP
}
while (($#)); do
  case "$1" in
    --dest|--link-path|--snapshot-run)
      (($# >= 2)) || { problem "$1" 'missing value'; exit 2; }
      case "$1" in
        --dest) dest="$2" ;;
        --link-path) link_path="$2"; make_link=1 ;;
        --snapshot-run) snapshot_run="$2" ;;
      esac
      shift 2 ;;
    --no-link) make_link=0; shift ;;
    --rebuild-generated) rebuild=1; shift ;;
    --help|-h) usage; exit 0 ;;
    *) problem "$1" 'unknown option'; exit 2 ;;
  esac
done
for command in curl sha256sum stat realpath mktemp mkdir mv ln; do
  command -v "$command" >/dev/null || { problem "$command" 'required command not found'; exit 2; }
done
dest="$(realpath -m -- "$dest")"
link_path="$(realpath -ms -- "$link_path")"
case "$dest" in /|"$ROOT"|"$ROOT"/*) problem "$dest" 'data must remain outside the checkout'; exit 2 ;; esac
if ((make_link)); then
  if [[ -L "$link_path" ]]; then
    [[ "$(realpath -m -- "$link_path")" == "$dest" ]] || { problem "$link_path" 'existing symlink points elsewhere; left unchanged'; exit 2; }
  elif [[ -e "$link_path" ]]; then
    problem "$link_path" 'existing real file or directory left unchanged; preserve it elsewhere or use --no-link'
    exit 2
  fi
fi
[[ "$snapshot_run" =~ ^[a-zA-Z0-9_-]+$ ]] || { problem "$snapshot_run" 'invalid snapshot run name'; exit 2; }

# Validate the entire manifest before any download; every URL must match its pinned path.
controlled=https://raw.githubusercontent.com/FuncularLabs/pubsmith/0ccdf27fc3f381f0897515e226ea485165c699b2/tests/fixtures/pub/
real=https://raw.githubusercontent.com/apache/poi/bf7c9b6a1490046517cbd8e1504e702a89be4bc9/test-data/publisher/
[[ -f "$MANIFEST" ]] || { problem "$MANIFEST" 'manifest missing'; exit 2; }
mapfile -t lines < "$MANIFEST"
declare -A seen=()
files=0; total_bytes=0; binaries=0; apache=0; truth=0; golden=0
for line in "${lines[@]}"; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  IFS=$'\t' read -r relative size sha url extra <<< "$line"
  [[ "$relative" =~ ^(pubsmith-fixtures/([a-zA-Z0-9_.-]+\.pub|truth/[a-zA-Z0-9_.-]+\.json|golden/[a-zA-Z0-9_.-]+\.png)|apache-pub/[a-zA-Z0-9_.-]+\.pub)$ && "$size" =~ ^[1-9][0-9]{0,8}$ && "$sha" =~ ^[a-f0-9]{64}$ && -z "$extra" ]] || { problem "$MANIFEST" "invalid row: $relative"; exit 2; }
  [[ -z "${seen[$relative]:-}" ]] || { problem "$relative" 'duplicate manifest entry'; exit 2; }
  seen[$relative]=1
  case "$relative" in
    apache-pub/*) expected="${real}${relative#apache-pub/}"; apache=$((apache + 1)) ;;
    pubsmith-fixtures/truth/*) expected="${controlled}${relative#pubsmith-fixtures/}"; truth=$((truth + 1)) ;;
    pubsmith-fixtures/golden/*) expected="${controlled}${relative#pubsmith-fixtures/}"; golden=$((golden + 1)) ;;
    *) expected="${controlled}${relative#pubsmith-fixtures/}"; binaries=$((binaries + 1)) ;;
  esac
  [[ "$url" == "$expected" ]] || { problem "$relative" 'URL is not the recorded pinned data URL'; exit 2; }
  files=$((files + 1)); total_bytes=$((total_bytes + size))
done
[[ "$files/$binaries/$apache/$truth/$golden" == '317/142/24/142/9' ]] || { problem "$MANIFEST" "wrong inventory: $files/$binaries/$apache/$truth/$golden"; exit 2; }
if ! error="$(mkdir -p -- "$dest" 2>&1)"; then problem "$dest" "$error"; exit 1; fi
temporary=''
cleanup() { if [[ -n "$temporary" ]]; then rm -f -- "$temporary" "$temporary.err"; fi; }
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
reason=''
verify() {
  local path="$1" wanted_size="$2" wanted_sha="$3" actual
  if [[ -L "$path" ]]; then reason='symlink in place of an input file'; return 1; fi
  if [[ ! -f "$path" ]]; then reason='missing or not a regular file'; return 1; fi
  if ! actual="$(stat -c %s -- "$path" 2>&1)"; then reason="$actual"; return 1; fi
  if [[ "$actual" != "$wanted_size" ]]; then reason="size mismatch: expected $wanted_size, got $actual"; return 1; fi
  if ! actual="$(sha256sum -- "$path" 2>&1)"; then reason="$actual"; return 1; fi
  actual="${actual%% *}"
  if [[ "$actual" != "$wanted_sha" ]]; then reason="SHA-256 mismatch: expected $wanted_sha, got $actual"; return 1; fi
}
restored=0; skipped=0; failed=0
for line in "${lines[@]}"; do
  [[ -z "$line" || "$line" == \#* ]] && continue
  IFS=$'\t' read -r relative size sha url <<< "$line"
  path="$dest/$relative"
  # Do not write through an existing parent symlink.
  parent="$dest"; blocked=0
  IFS=/ read -r -a components <<< "${relative%/*}"
  for component in "${components[@]}"; do
    parent="$parent/$component"
    if [[ -L "$parent" ]]; then problem "$relative" "parent is a symlink: $parent"; blocked=1; break; fi
  done
  if ((blocked)); then failed=$((failed + 1)); continue; fi
  if verify "$path" "$size" "$sha"; then skipped=$((skipped + 1)); continue; fi
  previous="$reason"
  if ! error="$(mkdir -p -- "$parent" 2>&1)"; then problem "$relative" "$error"; failed=$((failed + 1)); continue; fi
  if error="$(mktemp "$parent/.restore-niftydata.XXXXXXXX" 2>&1)"; then temporary="$error"; else problem "$relative" "$error"; failed=$((failed + 1)); continue; fi
  # Do not follow redirects: no request can leave the manifest's allowed host.
  if status="$(curl --fail --silent --show-error --proto '=https' \
      --connect-timeout 15 --max-time 120 --retry 3 --retry-all-errors \
      --retry-delay 1 --retry-max-time 180 --max-filesize "$size" \
      --write-out '%{http_code}' --output "$temporary" "$url" 2>"$temporary.err")"; then
    if [[ "$status" != 200 ]]; then reason="HTTP $status; expected 200 (redirects are not followed)";
    elif verify "$temporary" "$size" "$sha"; then
      if error="$(mv -fT -- "$temporary" "$path" 2>&1)"; then
        rm -f -- "$temporary.err"; temporary=''
        printf 'RESTORED %s: %s\n' "$relative" "$previous"
        restored=$((restored + 1)); continue
      else reason="$error"; fi
    fi
  else reason="$(<"$temporary.err")"; fi
  problem "$relative" "$reason"
  failed=$((failed + 1)); cleanup; temporary=''
done
printf 'Inputs: %s files, %s bytes; skipped=%s restored=%s failed=%s; elapsed=%ss\n' "$files" "$total_bytes" "$skipped" "$restored" "$failed" "$SECONDS"
((failed == 0)) || exit 1
if ((make_link)); then
  if [[ ! -L "$link_path" ]]; then
    if ! error="$(ln -sT -- "$dest" "$link_path" 2>&1)"; then problem "$link_path" "$error"; exit 1; fi
  fi
  [[ "$(realpath -m -- "$link_path")" == "$dest" ]] || { problem "$link_path" 'symlink no longer resolves to the destination'; exit 1; }
fi
if ((rebuild)); then
  [[ "$(realpath -m -- /tmp/niftydata)" == "$dest" ]] || { problem /tmp/niftydata 'generated runner requires this path to resolve to the destination'; exit 1; }
  [[ ! -e "$dest/pixel-snapshots/$snapshot_run" ]] || { problem "pixel-snapshots/$snapshot_run" 'snapshot run already exists; choose another name'; exit 1; }
  command -v cargo >/dev/null || { problem generated 'build commands unavailable; activate the existing development shell'; exit 1; }
  cd "$ROOT"
  export PUB_CORPUS="$dest/pubsmith-fixtures" PUB_REAL_CORPUS="$dest/apache-pub"
  # Generated checks use prepared dependencies; they must not add network hosts.
  export CARGO_NET_OFFLINE=true
  if ! PUB_PIXEL_MODE=publisher cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture; then problem pixel-report 'documented rebuild command failed'; exit 1; fi
  if ! PUB_PIXEL_MODE=snapshot PUB_PIXEL_RUN="$snapshot_run" cargo test --locked -p designcraft-cli --test pub_pixel_diff pub_pixel_diff -- --ignored --exact --nocapture; then problem "pixel-snapshots/$snapshot_run" 'documented rebuild command failed; partial run may remain'; exit 1; fi
  printf 'Generated pixel-report and pixel-snapshots/%s\n' "$snapshot_run"
fi
