#!/usr/bin/env python3
"""Fixed quick-v1 corpus. Sizes never adapt to the machine or measured speed."""
import argparse
import csv
import hashlib
import io
import json
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent
PROFILE = [
    (0, "24", []),
    (1, "499,500,2001", ["--layout", "line"]),
    (2, "48", []),
    (3, "256", ["--spiral", "all"]),
    (4, "32", []),
    (5, "192", []),
    (6, "10", []),
    (7, "8", []),
    (8, "18", []),
    (9, "256", ["--orientation", "all"]),
    (10, "160", []),
    (11, "1024", []),
    (12, "128", ["--outlier", "all"]),
    (13, "512", []),
]
COMMON = ["--int", "i32,i64", "--strategy", "all", "--mode", "all",
          "--seed", "42", "--warmup", "2", "--samples", "7", "--iterations", "1",
          "--multithreading", "false", "--budget-seconds", "0"]
KEY = ["test", "variant", "size", "int", "strategy", "mode", "seed"]
CONTRACT = ["schema", "fixture_version", "runner_source", "rustc", "target",
            "profile", "opt_level", "debug_info", "rustflags", "mt_compiled",
            "multithreading", "precision", "rule", "input_edges", "input_fingerprint",
            "output_contours", "output_edges", "output_area2", "warmup", "samples",
            "iterations"]


def read_rows(text):
    reader = csv.DictReader(io.StringIO(text))
    rows = list(reader)
    if not rows or not reader.fieldnames:
        raise ValueError("empty CSV")
    seen = set()
    for row in rows:
        if None in row or None in row.values() or row["validated"] != "true":
            raise ValueError("malformed or unvalidated CSV row")
        key = tuple(row[k] for k in KEY)
        if key in seen:
            raise ValueError(f"duplicate row: {key}")
        seen.add(key)
        samples = list(map(int, row["samples_ns"].split(";")))
        if (len(samples) != int(row["samples"])
                or int(statistics.median(samples)) != int(row["median_ns"])
                or min(samples) != int(row["min_ns"])
                or max(samples) != int(row["max_ns"])):
            raise ValueError(f"invalid sample statistics: {key}")
    return reader.fieldnames, rows


def compare(before, after):
    old = {tuple(r[k] for k in KEY): r for r in before}
    new = {tuple(r[k] for k in KEY): r for r in after}
    if old.keys() != new.keys():
        raise ValueError("comparison requires exactly the same cases, sizes, engines and modes")
    result = []
    for key, row in new.items():
        ref = old[key]
        changed = [k for k in CONTRACT if row[k] != ref[k]]
        if changed:
            raise ValueError(f"incompatible comparison {key}: {', '.join(changed)}")
        a, b = int(ref["median_ns"]), int(row["median_ns"])
        if a <= 0 or b <= 0:
            raise ValueError(f"nonpositive median: {key}")
        result.append({**{k: row[k] for k in KEY}, "baseline_ns": a, "current_ns": b,
                       "change_percent": f"{100 * (b / a - 1):.3f}",
                       "speedup": f"{a / b:.4f}"})
    return result


def capture(cmd):
    result = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    return result.stdout.strip() if result.returncode == 0 else "unavailable"


def write_csv(path, fields, rows):
    with path.open("x", newline="") as file:
        writer = csv.DictWriter(file, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path, help="new CSV; existing files are never overwritten")
    parser.add_argument("--compare", type=Path, help="baseline CSV with the same measurement contract")
    args = parser.parse_args()
    output = args.output.resolve()
    meta_path = output.with_suffix(".json")
    diff_path = output.with_suffix(".comparison.csv")
    for path in [output, meta_path, diff_path]:
        if path.exists():
            raise ValueError(f"refusing to overwrite {path}")
    before = read_rows(args.compare.read_text())[1] if args.compare else None
    output.parent.mkdir(parents=True, exist_ok=True)
    build = ["cargo", "build", "--release", "--offline", "--locked", "--bin", "rust_app",
             "--no-default-features", "--message-format=json-render-diagnostics"]
    start = time.perf_counter()
    built = subprocess.run(build, cwd=ROOT, stdout=subprocess.PIPE, text=True)
    if built.returncode:
        print(built.stdout, file=sys.stderr)
        raise RuntimeError("release build failed")
    build_seconds = time.perf_counter() - start
    artifacts = [json.loads(line) for line in built.stdout.splitlines() if line.startswith("{")]
    binary = next(a["executable"] for a in reversed(artifacts)
                  if a.get("reason") == "compiler-artifact" and a.get("executable")
                  and a["target"]["name"] == "rust_app")
    # First launch of a freshly linked executable can include OS verification.
    # Record it separately, before the 14 numbered tests and their warm-ups.
    start = time.perf_counter()
    subprocess.run([binary, "--help"], check=True, capture_output=True)
    launch_seconds = time.perf_counter() - start
    host = {"platform": platform.platform(), "machine": platform.machine(),
            "cpu": capture(["/usr/sbin/sysctl", "-n", "machdep.cpu.brand_string"]),
            "memory_bytes": capture(["/usr/sbin/sysctl", "-n", "hw.memsize"]),
            "logical_cpus": capture(["/usr/sbin/sysctl", "-n", "hw.ncpu"])}
    label = f"quick-v1 {host['cpu']}"
    metadata = {"profile": "quick-v1", "profile_cases": PROFILE, "common_args": COMMON,
                "profile_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                "git_revision": capture(["git", "rev-parse", "HEAD"]), "host": host,
                "started_unix": time.time(), "build_seconds": build_seconds,
                "first_launch_seconds": launch_seconds, "cases": [], "complete": False}
    rows, fields = [], None
    suite_start = time.perf_counter()
    try:
        for test, counts, options in PROFILE:
            command = [binary, "--test", str(test), "--count", counts, *COMMON,
                       *options, "--label", label]
            start = time.perf_counter()
            # Deliberately no timeout, adaptive sizing, or per-case time budget.
            run = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
            seconds = time.perf_counter() - start
            record = {"test": test, "command": command, "wall_seconds": seconds,
                      "exit_code": run.returncode, "stderr": run.stderr}
            metadata["cases"].append(record)
            if run.returncode or run.stderr:
                raise RuntimeError(f"test {test} failed or skipped data: {run.stderr}")
            header, case_rows = read_rows(run.stdout)
            variants = {1: ["line"], 3: ["float", "integer"],
                        9: ["horizontal", "vertical"], 12: ["cluster", "outlier"]}.get(test, ["default"])
            expected = {(str(test), v, n, i, s, m, "42")
                        for v in variants for n in counts.split(",") for i in ["i32", "i64"]
                        for s in ["auto", "list", "tree", "frag"] for m in ["cold", "reuse"]}
            if {tuple(r[k] for k in KEY) for r in case_rows} != expected:
                raise ValueError(f"test {test}: incomplete or unexpected matrix")
            if fields is not None and fields != header:
                raise ValueError("CSV header changed between tests")
            fields = header
            rows.extend(case_rows)
            record["rows"] = len(case_rows)
            record["input_edges"] = sorted({int(r["input_edges"]) for r in case_rows})
            print(f"test {test:2}: {seconds:.3f}s, {len(case_rows)} rows" +
                  (" (above baseline target; retained)" if seconds > 0.5 else ""), file=sys.stderr)
        metadata["complete"] = True
    finally:
        metadata["suite_seconds"] = time.perf_counter() - suite_start
        metadata["rows"] = len(rows)
        with meta_path.open("x") as file:
            json.dump(metadata, file, indent=2)
            file.write("\n")
    write_csv(output, fields, rows)
    if before is not None:
        changes = compare(before, rows)
        write_csv(diff_path, list(changes[0]), changes)
        print(f"Comparison: {diff_path} (positive change_percent = slower)", file=sys.stderr)
    print(f"Saved {len(rows)} rows: {output}\nSuite: {metadata['suite_seconds']:.3f}s", file=sys.stderr)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, RuntimeError, OSError, subprocess.CalledProcessError) as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
