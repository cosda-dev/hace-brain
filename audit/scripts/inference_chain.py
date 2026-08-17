#!/usr/bin/env python3
"""
audit/scripts/inference_chain.py  v2
Trace full inference chain: each link kiá»ƒm tra size + symbol + stub signal.
Output: per-step status + broken links + alpha3 readiness verdict.

Usage:
  python3 inference_chain.py
  python3 inference_chain.py --verbose
  python3 inference_chain.py --json
"""
import sys, re, json
from pathlib import Path

BASE    = Path("T:/hace/engine/hace")
VERBOSE = "--verbose" in sys.argv
AS_JSON = "--json"    in sys.argv

CHAIN = [
  {"id":"CLI",          "file":BASE/"brain/cli/src/prompt.rs",
   "need":["run_prompt","PromptArgs","run_algo"],
   "stub":[], "min_bytes":800, "produces":"PromptArgs{text,model,max_tokens}"},

  {"id":"BrainMasterRuntime", "file":BASE/"brain/master/src/lib.rs",
   "need":["execute","BrainMasterRuntime","InferenceEngine","InferRequest"],
   "stub":["SioOutcome::default()","let _ ="], "min_bytes":1200,
   "produces":"SioOutcome"},

  {"id":"InferenceEngine", "file":BASE/"brain/master/src/inference/engine.rs",
   "need":["InferenceEngine","infer","tokenizer","kv_cache"],
   "stub":["default()","todo!()","unimplemented!()"], "min_bytes":1500,
   "produces":"InferResult{logits,tokens,latency}",
   "critical":True},

  {"id":"GgufLoader",   "file":BASE/"brain/master/src/provider/gguf/loader.rs",
   "need":["GgufLoaderStd","loader_std","open","header"],
   "stub":["qwen2","\"291\"","hardcoded"], "min_bytes":1200,
   "produces":"LoadedModel{tensor_index,model_spec,mmap}"},

  {"id":"HacedleGgufLoaderStd", "file":BASE/"fem/hacedle/src/x/loader/gguf/loader_std.rs",
   "need":["GgufLoaderStd","load","header"], "stub":[],
   "min_bytes":500, "produces":"GgufHeader+TensorIndex"},

  {"id":"TensorIndex",  "file":BASE/"fem/hacedle/src/x/loader/gguf/tensor_index.rs",
   "need":["TensorIndex","build","lookup","offset"], "stub":[],
   "min_bytes":500, "produces":"name->TensorEntry map"},

  {"id":"BrainTokenizer", "file":BASE/"brain/master/src/tokenizer/mod.rs",
   "need":["tokenizers","BrainTokenizer","encode","decode","from_gguf"],
   "stub":["as u32","bytes()","ASCII"], "min_bytes":1000,
   "produces":"Vec<u32>  ('hello'->[9707])", "critical":True},

  {"id":"KvCache",      "file":BASE/"brain/master/src/runtime/kv_cache.rs",
   "need":["KvCache","prefill","alloc","n_layers","head_dim"],
   "stub":["todo!()","unimplemented!()","default()"], "min_bytes":1500,
   "produces":"[n_layers][seq][kv_heads][head_dim] f32"},

  {"id":"Prefill",      "file":BASE/"brain/master/src/runtime/prefill.rs",
   "need":["prefill_prompt","forward","embed","kv"],
   "stub":["todo!()","default()"], "min_bytes":800,
   "produces":"last_hidden: Vec<f32>[hidden_size]"},

  {"id":"HacedleEmbed", "file":BASE/"fem/hacedle/src/x/provider/candle/embed.rs",
   "need":["EmbeddingLayer","forward","embed"], "stub":[],
   "min_bytes":300, "produces":"Tensor[1,hidden]"},

  {"id":"HacedleLayer", "file":BASE/"fem/hacedle/src/x/provider/candle/layer.rs",
   "need":["TransformerLayer","forward","attention","ffn"],
   "stub":[], "min_bytes":500, "produces":"hidden_state Tensor"},

  {"id":"HacedleLmHead","file":BASE/"fem/hacedle/src/x/provider/candle/lmhead.rs",
   "need":["LmHead","forward","logits","vocab"],
   "stub":[], "min_bytes":500, "produces":"logits[vocab_size]"},

  {"id":"Sampler",      "file":BASE/"brain/master/src/runtime/logits.rs",
   "need":["greedy","argmax","topk","SampleStrategy"],
   "stub":["todo!()","default()"], "min_bytes":600,
   "produces":"next_token_id: u32"},
]

KNOWN_MISSING = {"BrainTokenizer", "Sampler", "Prefill", "KvCache"}

def scan(entry):
    p = Path(entry["file"])
    if not p.exists():
        return "MISSING", 0, [], entry["stub"]
    sz = p.stat().st_size
    if sz == 0:
        return "EMPTY", 0, [], []
    src = p.read_text(encoding="utf-8", errors="replace")
    found = [s for s in entry["need"] if s in src]
    stubs = [s for s in entry["stub"] if s in src]
    if stubs:
        status = "STUB"
    elif sz < entry["min_bytes"]:
        status = "SKELETON"
    elif len(found) >= max(1, len(entry["need"]) * 2 // 3):
        status = "OK"
    else:
        status = "PARTIAL"
    return status, sz, found, stubs

ICONS = {"OK":"âœ“","PARTIAL":"~","SKELETON":"â–³","STUB":"âš ","MISSING":"âœ—","EMPTY":"âˆ…"}

def main():
    rows, broken, critical_broken = [], [], []
    for e in CHAIN:
        st, sz, found, stubs = scan(e)
        known = e["id"] in KNOWN_MISSING
        rows.append({
            "id":st,"step":e["id"],"status":st,"size":sz,
            "found":found,"stubs":stubs,"critical":e.get("critical",False),
            "known_missing":known,"produces":e["produces"],
        })
        if st not in ("OK","PARTIAL") and not known:
            broken.append(e["id"])
            if e.get("critical"): critical_broken.append(e["id"])

    if AS_JSON:
        print(json.dumps(rows, indent=2)); return

    print("INFERENCE CHAIN  â€”  engine/hace/brain")
    print("="*78)
    for r in rows:
        icon = ICONS.get(r["status"],"?")
        km   = " [knownâˆ…]" if r["known_missing"] else ""
        crit = " â˜…" if r["critical"] else ""
        stub = f"  STUB:{r['stubs']}" if r["stubs"] else ""
        print(f"  {icon} {r['step']:<26} {r['status']:<10} {r['size']:>6}B  â†’  {r['produces'][:38]}{km}{crit}{stub}")
    print("="*78)
    print(f"BROKEN (non-known): {broken or 'NONE'}")
    print(f"CRITICAL BROKEN:    {critical_broken or 'NONE'}")
    ready = not broken and not critical_broken
    print(f"ALPHA-3 CHAIN OK:   {'YES' if ready else 'NO'}")
    if not ready:
        print()
        if "InferenceEngine" in broken:
            print("  â˜… inference/engine.rs unchanged â€” wire tokenizer+kv+kernel NOW")
        if "GgufLoader" in broken:
            print("  âš  loader.rs has hardcoded stub â€” delegate to hacedle::GgufLoaderStd")
        if "BrainTokenizer" in broken or "BrainTokenizer" in [r["step"] for r in rows if r["status"]=="STUB"]:
            print("  â˜… tokenizer: add tokenizers crate dep, rewrite encode/decode")
    print("\n  âœ“=OK  ~=partial  â–³=skeleton  âš =stub  âœ—=missing  âˆ…=empty  â˜…=critical")

if __name__ == "__main__":
    main()
