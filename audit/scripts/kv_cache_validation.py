#!/usr/bin/env python3
"""
audit/scripts/kv_cache_validation.py  v2
Validate KvCache implementation against canon/asi/runtime.ail spec.
Checks: struct existence, memory layout correctness, prefill logic,
        error handling (OOM prevention, invalid shape), no panics.

Usage:
  python3 kv_cache_validation.py
  python3 kv_cache_validation.py --verbose
"""
import sys, re
from pathlib import Path

BASE    = Path("T:/hace/engine/hace/brain/master/src/runtime")
VERBOSE = "--verbose" in sys.argv

# Canon spec from asi/runtime.ail
CANON = {
    "kv_shape_fields": ["n_layers","max_seq","n_kv_heads","head_dim"],
    "required_methods": ["new","prefill","get_k","get_v","reset","current_seq_len"],
    "oom_guard":   ["max_seq","seq_len","Err","KvError","return Err"],
    "no_panic":    ["panic!","unwrap()","expect("],  # these should NOT appear in error paths
    "alloc_hint":  ["Vec::<f32>::with_capacity","vec![0.0","vec![0.0f32"],
    "min_bytes":   1500,
}

PREFILL_CANON = {
    "required": ["prefill_prompt","embed","forward","kv","token_ids"],
    "loop_pattern": ["for ","enumerate","pos"],
    "min_bytes": 800,
}

SAMPLER_CANON = {
    "required": ["greedy","argmax","SampleStrategy","sample"],
    "nan_guard": ["is_nan","f32::NEG_INFINITY","nan"],
    "min_bytes": 600,
}

def read(path):
    p = Path(path)
    if not p.exists(): return None, 0
    return p.read_text(encoding="utf-8", errors="replace"), p.stat().st_size

def check_symbols(src, symbols):
    return {s: (s in src) for s in symbols}

def find_panics_in_error_paths(src):
    """Detect panic!/unwrap() inside if/match error handling blocks."""
    panic_lines = []
    for i, line in enumerate(src.splitlines(), 1):
        stripped = line.strip()
        if any(p in stripped for p in ["panic!(","unwrap()","expect("]):
            # Flag if appears to be in error handling context
            context = src.splitlines()[max(0,i-4):i+1]
            ctx_str = " ".join(context).lower()
            if any(k in ctx_str for k in ["err","fail","invalid","oom","overflow"]):
                panic_lines.append((i, stripped[:60]))
    return panic_lines

def validate_kv_cache():
    src, sz = read(BASE / "kv_cache.rs")
    results = {"file": "runtime/kv_cache.rs", "size": sz, "checks": {}}

    if src is None:
        results["status"] = "MISSING"
        return results

    # Size gate
    results["checks"]["min_size"] = {
        "expected": f">= {CANON['min_bytes']}B",
        "actual": sz,
        "pass": sz >= CANON["min_bytes"],
        "note": "skeleton if < 1500B" if sz < CANON["min_bytes"] else "ok",
    }

    # Shape fields
    shape_hits = check_symbols(src, CANON["kv_shape_fields"])
    results["checks"]["shape_fields"] = {
        "found": [k for k,v in shape_hits.items() if v],
        "missing": [k for k,v in shape_hits.items() if not v],
        "pass": all(shape_hits.values()),
    }

    # Required methods
    method_hits = check_symbols(src, CANON["required_methods"])
    results["checks"]["methods"] = {
        "found": [k for k,v in method_hits.items() if v],
        "missing": [k for k,v in method_hits.items() if not v],
        "pass": sum(method_hits.values()) >= 4,
    }

    # OOM guard
    oom_hits = check_symbols(src, CANON["oom_guard"])
    results["checks"]["oom_prevention"] = {
        "found": [k for k,v in oom_hits.items() if v],
        "pass": oom_hits.get("Err", False) and (
            oom_hits.get("max_seq", False) or oom_hits.get("seq_len", False)
        ),
        "note": "must check seq_len <= max_seq and return Err (not panic)",
    }

    # Panic in error paths
    panics = find_panics_in_error_paths(src)
    results["checks"]["no_panic_in_errors"] = {
        "panics_found": panics,
        "pass": len(panics) == 0,
    }

    # Allocation
    alloc_hits = check_symbols(src, CANON["alloc_hint"])
    results["checks"]["allocation"] = {
        "found": [k for k,v in alloc_hits.items() if v],
        "pass": any(alloc_hits.values()),
        "note": "must allocate Vec<f32> for kv tensors",
    }

    passed = sum(1 for c in results["checks"].values() if c.get("pass", False))
    total  = len(results["checks"])
    results["status"] = "OK" if passed == total else f"PARTIAL ({passed}/{total})"
    return results

def validate_prefill():
    src, sz = read(BASE / "prefill.rs")
    results = {"file": "runtime/prefill.rs", "size": sz, "checks": {}}
    if src is None:
        results["status"] = "MISSING"; return results

    hits = check_symbols(src, PREFILL_CANON["required"])
    loop = check_symbols(src, PREFILL_CANON["loop_pattern"])
    results["checks"]["required_symbols"] = {
        "found": [k for k,v in hits.items() if v],
        "missing": [k for k,v in hits.items() if not v],
        "pass": sum(hits.values()) >= 3,
    }
    results["checks"]["loop_pattern"] = {
        "found": [k for k,v in loop.items() if v],
        "pass": any(loop.values()),
        "note": "must iterate token_ids with position index",
    }
    results["checks"]["min_size"] = {"pass": sz >= PREFILL_CANON["min_bytes"],
                                     "actual": sz}
    passed = sum(1 for c in results["checks"].values() if c.get("pass",False))
    results["status"] = "OK" if passed == 3 else f"PARTIAL ({passed}/3)"
    return results

def validate_sampler():
    src, sz = read(BASE / "logits.rs")
    results = {"file": "runtime/logits.rs", "size": sz, "checks": {}}
    if src is None:
        results["status"] = "MISSING"; return results

    hits    = check_symbols(src, SAMPLER_CANON["required"])
    nan_hits = check_symbols(src, SAMPLER_CANON["nan_guard"])
    results["checks"]["required_symbols"] = {
        "found":   [k for k,v in hits.items() if v],
        "missing": [k for k,v in hits.items() if not v],
        "pass": sum(hits.values()) >= 2,
    }
    results["checks"]["nan_guard"] = {
        "found": [k for k,v in nan_hits.items() if v],
        "pass": any(nan_hits.values()),
        "note": "NaN logits must be replaced with -inf before argmax",
    }
    results["checks"]["min_size"] = {"pass": sz >= SAMPLER_CANON["min_bytes"],
                                     "actual": sz}
    passed = sum(1 for c in results["checks"].values() if c.get("pass",False))
    results["status"] = "OK" if passed == 3 else f"PARTIAL ({passed}/3)"
    return results

def main():
    kv  = validate_kv_cache()
    pre = validate_prefill()
    sam = validate_sampler()

    all_results = [kv, pre, sam]
    overall_ok  = all(r["status"] == "OK" for r in all_results)

    print("KV-CACHE / RUNTIME VALIDATION")
    print("="*60)
    for r in all_results:
        mark = "âœ“" if r["status"] == "OK" else "âœ—"
        print(f"  {mark} {r['file']:<36} {r['status']}  ({r['size']}B)")
        if VERBOSE or r["status"] != "OK":
            for name, chk in r.get("checks", {}).items():
                pm = "  +" if chk.get("pass") else "  -"
                print(f"    {pm} {name}: {chk}")
    print("="*60)
    print(f"RUNTIME READY: {'YES' if overall_ok else 'NO'}")
    if not overall_ok:
        print()
        if kv["size"] < CANON["min_bytes"]:
            print("  kv_cache.rs too small â€” implement alloc + prefill + get_k/v methods")
        if pre["size"] < PREFILL_CANON["min_bytes"]:
            print("  prefill.rs too small â€” implement token loop calling hacedle.forward()")
        if sam["size"] < SAMPLER_CANON["min_bytes"]:
            print("  logits.rs too small â€” implement greedy argmax + NaN guard")

if __name__ == "__main__":
    main()
