#!/usr/bin/env python3
"""
CSA Alpha-3 Inference Chain Tracer
Trace tung buoc inference chain tu CLI den logits.
Kiem tra moi link trong chuoi co ton tai va co noi dung thuc hay khong.

Usage: python3 inference_chain_trace.py [--verbose]
"""

import os, re, json, sys
from pathlib import Path

BASE = Path("T:/hace/engine/hace")
VERBOSE = "--verbose" in sys.argv

# â”€â”€â”€ Chain definition â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

CHAIN = [
    {
        "step": "CLI",
        "desc": "hace brain prompt -> parse args",
        "file": BASE / "brain/cli/src/prompt.rs",
        "expected_symbols": ["run_prompt", "PromptArgs", "run_algo"],
        "produces": "InferRequest { text, model_path, max_tokens }",
    },
    {
        "step": "BrainMasterRuntime",
        "desc": "execute(sio) -> route to inference",
        "file": BASE / "brain/master/src/lib.rs",
        "expected_symbols": ["execute", "BrainMasterRuntime", "InferenceEngine"],
        "produces": "SioOutcome { tokens, latency }",
        "stub_signal": "SioOutcome::default()",  # if this appears -> STUB
    },
    {
        "step": "GgufLoader",
        "desc": "load GGUF -> TensorIndex + ModelSpec",
        "file": BASE / "brain/master/src/provider/gguf/loader.rs",
        "expected_symbols": ["GgufLoaderStd", "loader_std", "hacedle"],
        "stub_signals": ['"qwen2"', "291", "hardcoded"],
        "produces": "LoadedModel { tensor_index, model_spec }",
    },
    {
        "step": "GgufLoaderStd (hacedle)",
        "desc": "Real GGUF parser in hacedle",
        "file": BASE / "fem/hacedle/src/x/loader/gguf/loader_std.rs",
        "expected_symbols": ["GgufLoaderStd", "load", "header"],
        "produces": "GgufHeader + TensorIndex",
    },
    {
        "step": "TensorIndex (hacedle)",
        "desc": "Name -> offset tensor map",
        "file": BASE / "fem/hacedle/src/x/loader/gguf/tensor_index.rs",
        "expected_symbols": ["TensorIndex", "build", "lookup"],
        "produces": "HashMap<name, TensorEntry>",
    },
    {
        "step": "Tokenizer",
        "desc": "text -> token_ids",
        "file": BASE / "brain/master/src/tokenizer/mod.rs",
        "expected_symbols": ["BrainTokenizer", "encode", "decode"],
        "produces": "Vec<u32>",
        "known_missing": True,
    },
    {
        "step": "EmbeddingLayer (hacedle)",
        "desc": "token_ids -> f32 embeddings",
        "file": BASE / "fem/hacedle/src/x/provider/candle/embed.rs",
        "expected_symbols": ["EmbeddingLayer", "forward", "embed"],
        "produces": "Tensor[1, seq, hidden_size]",
    },
    {
        "step": "KvCache",
        "desc": "KV cache for attention",
        "file": BASE / "brain/master/src/runtime/kv_cache.rs",
        "expected_symbols": ["KvCache", "prefill"],
        "produces": "KvCache { k, v per layer }",
        "known_missing": True,
    },
    {
        "step": "TransformerLayer (hacedle)",
        "desc": "Attention + FFN per block",
        "file": BASE / "fem/hacedle/src/x/provider/candle/layer.rs",
        "expected_symbols": ["TransformerLayer", "forward", "attention"],
        "produces": "hidden_state Tensor",
    },
    {
        "step": "LmHead (hacedle)",
        "desc": "hidden -> logits[vocab_size]",
        "file": BASE / "fem/hacedle/src/x/provider/candle/lmhead.rs",
        "expected_symbols": ["LmHead", "forward", "logits"],
        "produces": "Tensor[vocab_size]",
    },
    {
        "step": "Sampler",
        "desc": "logits -> next token (greedy/topk)",
        "file": BASE / "brain/master/src/runtime/sampler.rs",
        "expected_symbols": ["greedy", "argmax", "topk"],
        "produces": "token_id: u32",
        "known_missing": True,
    },
]

# â”€â”€â”€ Symbol scanner â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def scan_file(path: Path, symbols: list, stub_signals: list = None) -> dict:
    if not path.exists():
        return {"status": "MISSING", "size": 0, "found": [], "stubs": []}
    size = path.stat().st_size
    if size == 0:
        return {"status": "EMPTY", "size": 0, "found": [], "stubs": []}
    try:
        src = path.read_text(encoding="utf-8", errors="replace")
    except Exception as e:
        return {"status": "READ_ERR", "error": str(e), "found": [], "stubs": []}

    found = [s for s in symbols if s in src]
    stubs = [s for s in (stub_signals or []) if s in src]

    if stubs:
        status = "HARDCODED_STUB"
    elif len(found) == 0:
        status = "EMPTY_SYMBOLS"
    elif len(found) >= len(symbols) // 2:
        status = "OK"
    else:
        status = "PARTIAL"

    return {
        "status":     status,
        "size":       size,
        "found":      found,
        "missing":    [s for s in symbols if s not in src],
        "stubs":      stubs,
    }

# â”€â”€â”€ Main trace â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def main():
    results = []
    broken_links = []

    for step in CHAIN:
        r = scan_file(
            Path(step["file"]),
            step.get("expected_symbols", []),
            step.get("stub_signals", []),
        )

        entry = {
            "step":         step["step"],
            "desc":         step["desc"],
            "file":         str(step["file"]).replace(str(BASE), ""),
            "status":       r["status"],
            "size":         r["size"],
            "symbols_found":r["found"],
            "known_missing":step.get("known_missing", False),
            "produces":     step["produces"],
        }

        if r.get("stubs"):
            entry["stub_signals_found"] = r["stubs"]

        if r["status"] in ("MISSING", "EMPTY", "HARDCODED_STUB", "EMPTY_SYMBOLS"):
            if not step.get("known_missing"):
                broken_links.append(entry["step"])

        results.append(entry)

    # Output
    chain_ok = len(broken_links) == 0

    if VERBOSE:
        print(json.dumps(results, indent=2))
    else:
        print("INFERENCE CHAIN TRACE")
        print("=" * 70)
        for r in results:
            mark = {
                "OK":               "âœ“",
                "PARTIAL":          "~",
                "MISSING":          "âœ—",
                "EMPTY":            "âˆ…",
                "HARDCODED_STUB":   "âš ",
                "EMPTY_SYMBOLS":    "?",
                "READ_ERR":         "!",
            }.get(r["status"], "?")

            km = " [known-missing]" if r["known_missing"] else ""
            stub_note = f"  STUB:{r.get('stub_signals_found','')}" if r.get("stub_signals_found") else ""
            print(f"  {mark} {r['step']:<28} {r['status']:<18} {r['size']:>6}B  ->  {r['produces'][:40]}{km}{stub_note}")

        print("=" * 70)
        print(f"BROKEN LINKS (non-known-missing): {broken_links if broken_links else 'NONE'}")
        print(f"CHAIN COMPLETE: {'YES' if chain_ok else 'NO'}")
        print()
        print("LEGEND: âœ“=OK  ~=partial  âœ—=missing  âˆ…=empty  âš =hardcoded_stub")
        print()
        print("CRITICAL PATH TO FIRST TOKEN:")
        print("  [1] Fix brain/master/gguf/loader.rs -> delegate to hacedle")
        print("  [2] Create brain/master/src/tokenizer/ (tokenizers crate)")
        print("  [3] Create brain/master/src/runtime/kv_cache.rs")
        print("  [4] Wire brain/master/src/inference/engine.rs -> hacedle candle chain")
        print("  [5] Create greedy sampler (argmax logits)")

if __name__ == "__main__":
    main()
