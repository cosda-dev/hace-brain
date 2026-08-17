#!/usr/bin/env python3
"""
CSA Alpha-3 Gap Checker
Kiem tra tinh trang thuc te cua toan bo Alpha-3 asset chain.
Usage: python3 alpha3_gap_check.py [--gguf PATH]
"""

import os, sys, struct, json
from pathlib import Path
from typing import Optional

BASE = Path("T:/hace/engine/hace")

# â”€â”€â”€ File existence + size checks â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def check_file(path: str, min_bytes: int = 10, label: str = "") -> dict:
    p = Path(path)
    exists = p.exists()
    size = p.stat().st_size if exists else 0
    real = exists and size >= min_bytes
    return {
        "path":    str(p),
        "label":   label,
        "exists":  exists,
        "size":    size,
        "real":    real,
        "status":  "OK" if real else ("EMPTY" if exists else "MISSING"),
    }

ASSETS = [
    # â”€â”€ hacedle GGUF real loader â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"fem/hacedle/src/x/loader/gguf/loader.rs",       500, "hacedle.gguf.loader"),
    (BASE/"fem/hacedle/src/x/loader/gguf/loader_std.rs",   200, "hacedle.gguf.loader_std"),
    (BASE/"fem/hacedle/src/x/loader/gguf/header.rs",       200, "hacedle.gguf.header"),
    (BASE/"fem/hacedle/src/x/loader/gguf/metadata.rs",      50, "hacedle.gguf.metadata"),
    (BASE/"fem/hacedle/src/x/loader/gguf/tensor_index.rs", 500, "hacedle.gguf.tensor_index"),
    (BASE/"fem/hacedle/src/x/loader/gguf/tensor_projection.rs", 300, "hacedle.gguf.tensor_proj"),
    (BASE/"fem/hacedle/src/x/loader/gguf/mmap.rs",         50,  "hacedle.gguf.mmap"),
    (BASE/"fem/hacedle/src/x/loader/gguf/model_spec.rs",  200,  "hacedle.gguf.model_spec"),
    (BASE/"fem/hacedle/src/x/loader/gguf/quant_router.rs", 100, "hacedle.gguf.quant_router"),
    (BASE/"fem/hacedle/src/x/loader/dequant/q4_k.rs",      200, "hacedle.dequant.q4k"),

    # â”€â”€ hacedle candle inference chain â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"fem/hacedle/src/x/provider/candle/inference.rs", 200, "hacedle.candle.inference"),
    (BASE/"fem/hacedle/src/x/provider/candle/layer.rs",     200, "hacedle.candle.layer"),
    (BASE/"fem/hacedle/src/x/provider/candle/lmhead.rs",    300, "hacedle.candle.lmhead"),
    (BASE/"fem/hacedle/src/x/provider/candle/embed.rs",     100, "hacedle.candle.embed"),

    # â”€â”€ hacedle kvm (kv cache) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"fem/hacedle/src/x/loader/kvm/mod.rs",            50,  "hacedle.kvm"),

    # â”€â”€ brain/master GGUF bridge â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"brain/master/src/provider/gguf/loader.rs",       50,  "brain.gguf.loader"),
    (BASE/"brain/master/src/provider/gguf/metadata.rs",     50,  "brain.gguf.metadata"),
    (BASE/"brain/master/src/provider/gguf/mmap.rs",         10,  "brain.gguf.mmap"),
    (BASE/"brain/master/src/provider/gguf/tensor.rs",       50,  "brain.gguf.tensor"),

    # â”€â”€ brain/master kernel + inference â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"brain/master/src/kernel/hacedle.rs",             100, "brain.kernel.hacedle"),
    (BASE/"brain/master/src/inference/engine.rs",           100, "brain.inference.engine"),
    (BASE/"brain/master/src/inference/request.rs",           50, "brain.inference.request"),
    (BASE/"brain/master/src/inference/response.rs",          50, "brain.inference.response"),
    (BASE/"brain/master/src/session/context.rs",            100, "brain.session.context"),

    # â”€â”€ TOKENIZER â€” expected missing â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"brain/master/src/tokenizer/mod.rs",               10, "brain.tokenizer.mod"),
    (BASE/"brain/master/src/tokenizer/gguf_tok.rs",          10, "brain.tokenizer.gguf"),

    # â”€â”€ KV CACHE runtime â€” expected missing â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"brain/master/src/runtime/kv_cache.rs",            10, "brain.runtime.kv_cache"),
    (BASE/"brain/master/src/runtime/prefill.rs",             10, "brain.runtime.prefill"),

    # â”€â”€ CLI â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    (BASE/"brain/cli/src/lib.rs",                            10, "brain.cli.lib"),
    (BASE/"brain/cli/src/command.rs",                       200, "brain.cli.command"),
    (BASE/"brain/cli/src/brain.rs",                         200, "brain.cli.brain"),
    (BASE/"brain/cli/src/prompt.rs",                        200, "brain.cli.prompt"),
    (BASE/"brain/cli/src/model.rs",                         200, "brain.cli.model"),
]

# â”€â”€â”€ GGUF magic check â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def check_gguf_file(path: str) -> dict:
    p = Path(path)
    if not p.exists():
        return {"status": "FILE_NOT_FOUND", "path": path}
    try:
        with open(p, "rb") as f:
            magic = f.read(4)
            if magic != b"GGUF":
                return {"status": "NOT_GGUF", "magic_hex": magic.hex(), "path": path}
            version = struct.unpack("<I", f.read(4))[0]
            tensor_count = struct.unpack("<Q", f.read(8))[0]
            kv_count     = struct.unpack("<Q", f.read(8))[0]
            size_mb = p.stat().st_size / 1_048_576
        return {
            "status":       "VALID_GGUF",
            "path":         str(p),
            "size_mb":      round(size_mb, 2),
            "gguf_version": version,
            "tensor_count": tensor_count,
            "kv_count":     kv_count,
        }
    except Exception as e:
        return {"status": "READ_ERROR", "error": str(e), "path": path}

# â”€â”€â”€ Stub detection (brain/master/gguf/loader.rs heuristic) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def detect_hardcoded_stub(loader_path: str) -> dict:
    p = Path(loader_path)
    if not p.exists():
        return {"stub_detected": None, "reason": "file not found"}
    try:
        src = p.read_text(encoding="utf-8", errors="replace")
        hardcoded = '"qwen2"' in src or '"291"' in src or "291," in src
        delegates = "hacedle" in src or "loader_std" in src or "GgufLoaderStd" in src
        return {
            "stub_detected": hardcoded and not delegates,
            "hardcoded_arch": hardcoded,
            "delegates_to_hacedle": delegates,
            "verdict": (
                "HARDCODED_STUB â€” khong goi hacedle" if (hardcoded and not delegates)
                else "DELEGATES_OK" if delegates
                else "UNKNOWN"
            ),
        }
    except Exception as e:
        return {"stub_detected": None, "error": str(e)}

# â”€â”€â”€ Cargo.toml tokenizers dep check â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def check_tokenizers_dep(cargo_path: str) -> dict:
    p = Path(cargo_path)
    if not p.exists():
        return {"found": False, "reason": "Cargo.toml not found"}
    src = p.read_text(encoding="utf-8", errors="replace")
    found = "tokenizers" in src
    return {"found": found, "status": "OK" if found else "MISSING â€” add tokenizers dep"}

# â”€â”€â”€ Run all checks â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def main():
    gguf_path = None
    for i, arg in enumerate(sys.argv[1:]):
        if arg == "--gguf" and i + 1 < len(sys.argv) - 1:
            gguf_path = sys.argv[i + 2]

    results = {"assets": [], "stub_audit": {}, "tokenizer_dep": {}, "gguf_file": {}}

    # Asset existence
    ok = miss = empty = 0
    for path, min_b, label in ASSETS:
        r = check_file(path, min_b, label)
        results["assets"].append(r)
        if r["status"] == "OK":       ok += 1
        elif r["status"] == "EMPTY":  empty += 1
        else:                         miss += 1

    # Stub detection
    results["stub_audit"] = detect_hardcoded_stub(
        str(BASE / "brain/master/src/provider/gguf/loader.rs")
    )

    # Tokenizers dep
    results["tokenizer_dep"] = check_tokenizers_dep(
        str(BASE / "brain/master/Cargo.toml")
    )

    # GGUF file check if provided
    if gguf_path:
        results["gguf_file"] = check_gguf_file(gguf_path)
    else:
        results["gguf_file"] = {"note": "no --gguf path provided â€” skipped"}

    # Summary
    results["summary"] = {
        "total_assets":  len(ASSETS),
        "ok":            ok,
        "missing":       miss,
        "empty":         empty,
        "alpha3_ready":  False,
        "blockers": [
            r["label"] for r in results["assets"]
            if r["status"] in ("MISSING", "EMPTY")
            and "tokenizer" not in r["label"]
            and "runtime" not in r["label"]
        ][:5],
    }

    # Missing critical (not tokenizer/runtime which are known missing)
    critical_missing = [
        r["label"] for r in results["assets"]
        if r["status"] == "MISSING"
        and r["label"] not in (
            "brain.tokenizer.mod", "brain.tokenizer.gguf",
            "brain.runtime.kv_cache", "brain.runtime.prefill"
        )
    ]
    results["summary"]["critical_missing"] = critical_missing
    results["summary"]["alpha3_ready"] = (
        miss == 4 and ok >= 25  # only the 4 known-missing expected
    )

    print(json.dumps(results, indent=2))

    # Human-readable summary
    print("\n" + "="*60)
    print(f"ALPHA-3 ASSET CHECK: {ok}/{len(ASSETS)} OK, {miss} MISSING, {empty} EMPTY")
    print(f"STUB AUDIT:          {results['stub_audit'].get('verdict', 'N/A')}")
    print(f"TOKENIZERS DEP:      {results['tokenizer_dep'].get('status', 'N/A')}")
    if gguf_path:
        print(f"GGUF FILE:           {results['gguf_file'].get('status', 'N/A')}")
    if critical_missing:
        print(f"CRITICAL MISSING:    {critical_missing}")
    print("ALPHA-3 READY:", "YES" if results["summary"]["alpha3_ready"] else "NO")
    print("="*60)

if __name__ == "__main__":
    main()
