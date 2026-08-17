#!/usr/bin/env python3
"""
audit/scripts/duplication_check.py  v1
Phat hien vi pham reuse: brain/master reimplementing ops da co trong hacedle.
Canon rule S-05 (audits/struct-rules.ail).

Usage: python3 duplication_check.py [--verbose]
"""
import sys, re
from pathlib import Path

BASE    = Path("T:/hace/engine/hace")
VERBOSE = "--verbose" in sys.argv

# Canonical sources (hacedle owns these â€” brain MUST NOT reimplement)
CANONICAL = {
    "gguf_magic_parse": {
        "canonical": "fem/hacedle/src/x/loader/gguf/header.rs",
        "signals":   [r'b"GGUF"', r'magic.*==', r'0x47475546'],
        "scan_paths": ["brain/master/src/provider/gguf/loader.rs",
                       "brain/master/src/provider/gguf/mod.rs"],
    },
    "rmsnorm": {
        "canonical": "fem/hacedle/src/ops/rmsnorm.rs",
        "signals":   [r'rms_norm', r'RmsNorm', r'fn rmsnorm', r'norm_weight.*f32'],
        "scan_paths": ["brain/master/src/", "brain/runtime/src/"],
    },
    "rope_positional": {
        "canonical": "fem/hacedle/src/ops/rope.rs",
        "signals":   [r'fn rope', r'apply_rope', r'cos_sin', r'theta.*pos'],
        "scan_paths": ["brain/master/src/", "brain/runtime/src/"],
    },
    "attention_qkv": {
        "canonical": "fem/hacedle/src/ops/attention.rs",
        "signals":   [r'fn attention', r'softmax.*scores', r'n_heads.*head_dim'],
        "scan_paths": ["brain/master/src/", "brain/runtime/src/"],
    },
    "silu_activation": {
        "canonical": "fem/hacedle/src/ops/silu.rs",
        "signals":   [r'fn silu', r'x \* sigmoid', r'1\.0.*exp'],
        "scan_paths": ["brain/master/src/", "brain/runtime/src/"],
    },
    "q4k_dequant": {
        "canonical": "fem/hacedle/src/x/loader/dequant/q4_k.rs",
        "signals":   [r'Q4_K', r'q4k', r'dequant.*q4', r'qs.*xs.*scales'],
        "scan_paths": ["brain/master/src/"],
    },
    "tensor_index": {
        "canonical": "fem/hacedle/src/x/loader/gguf/tensor_index.rs",
        "signals":   [r'struct TensorIndex', r'fn build_index', r'BTreeMap.*tensor'],
        "scan_paths": ["brain/master/src/"],
        "note": "brain/master/src/provider/gguf/tensor.rs was removed (good)"
    },
}

def scan_dir(directory: Path, signals: list) -> list:
    """Find files in directory that contain any signal pattern."""
    hits = []
    p = BASE / directory
    if not p.exists(): return hits
    for f in p.rglob("*.rs"):
        try:
            src = f.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        matched = [s for s in signals if re.search(s, src)]
        if matched:
            hits.append({"file": str(f).replace(str(BASE)+"/",""),
                         "matched": matched, "size": f.stat().st_size})
    return hits

def main():
    violations = []
    clean = []

    for rule_id, rule in CANONICAL.items():
        canonical_exists = (BASE / rule["canonical"]).exists()
        all_hits = []
        for scan_path in rule["scan_paths"]:
            hits = scan_dir(Path(scan_path), rule["signals"])
            all_hits.extend(hits)

        if all_hits:
            violations.append({
                "rule":      rule_id,
                "canonical": rule["canonical"],
                "canonical_exists": canonical_exists,
                "duplicate_in": all_hits,
                "note": rule.get("note",""),
            })
        else:
            clean.append(rule_id)

    print("DUPLICATION CHECK â€” canon/audits/struct-rules.ail S-05")
    print("="*62)
    if violations:
        for v in violations:
            print(f"\n  VIOLATION: {v['rule']}")
            print(f"    canonical: {v['canonical']}  (exists={v['canonical_exists']})")
            for hit in v["duplicate_in"]:
                print(f"    DUPLICATE: {hit['file']}  ({hit['size']}B)")
                if VERBOSE:
                    print(f"      signals: {hit['matched']}")
            if v["note"]:
                print(f"    note: {v['note']}")
    else:
        print("  No duplication violations found.")

    print(f"\n  CLEAN rules: {clean}")
    print(f"  VIOLATIONS:  {len(violations)}")
    print("="*62)
    sys.exit(1 if violations else 0)

if __name__ == "__main__":
    main()
