#!/usr/bin/env python3
"""
audit/scripts/gguf_verify.py  v2
Kiem tra GGUF file: magic, version, tensor_count, metadata KV.
Phat hien hardcoded stub trong brain/master/src/provider/gguf/loader.rs.

Usage:
  python3 gguf_verify.py <model.gguf>
  python3 gguf_verify.py <model.gguf> --tensors
  python3 gguf_verify.py <model.gguf> --metadata
  python3 gguf_verify.py --stub-check   # chi kiem tra loader.rs co stub khong
"""
import sys, struct, os, re
from pathlib import Path

BASE     = Path("T:/hace/engine/hace")
LOADER   = BASE / "brain/master/src/provider/gguf/loader.rs"
GGML_T   = {0:"F32",1:"F16",2:"Q4_0",3:"Q4_1",6:"Q5_0",7:"Q5_1",
            8:"Q8_0",10:"Q2_K",11:"Q3_K",12:"Q4_K",13:"Q5_K",14:"Q6_K",29:"BF16"}

# â”€â”€ GGUF reader â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def _rs(f):
    n = struct.unpack("<Q", f.read(8))[0]
    if n > 1_000_000: raise ValueError(f"string too long: {n}")
    return f.read(n).decode("utf-8", errors="replace")

def _rv(f, t):
    if t == 8:  return _rs(f)
    if t == 0:  return struct.unpack("<B", f.read(1))[0]
    if t == 1:  return struct.unpack("<b", f.read(1))[0]
    if t == 2:  return struct.unpack("<H", f.read(2))[0]
    if t == 3:  return struct.unpack("<h", f.read(2))[0]
    if t == 4:  return struct.unpack("<I", f.read(4))[0]
    if t == 5:  return struct.unpack("<i", f.read(4))[0]
    if t == 6:  return struct.unpack("<f", f.read(4))[0]
    if t == 7:  return bool(struct.unpack("<B", f.read(1))[0])
    if t == 10: return struct.unpack("<Q", f.read(8))[0]
    if t == 11: return struct.unpack("<q", f.read(8))[0]
    if t == 12: return struct.unpack("<d", f.read(8))[0]
    if t == 9:
        et = struct.unpack("<I", f.read(4))[0]
        n  = struct.unpack("<Q", f.read(8))[0]
        sample = []
        for i in range(n):
            v = _rv(f, et)
            if i < 3: sample.append(v)
        return sample + ([f"...({n} total)"] if n > 3 else [])
    raise ValueError(f"unknown vtype {t}")

def parse_gguf(path, tensors=False, metadata=False):
    p = Path(path)
    if not p.exists(): return {"error": f"not found: {path}"}
    size_mb = round(p.stat().st_size / 1_048_576, 2)
    with open(p, "rb") as f:
        magic = f.read(4)
        if magic != b"GGUF": return {"error": f"bad magic: {magic.hex()}"}
        ver = struct.unpack("<I", f.read(4))[0]
        tc  = struct.unpack("<Q", f.read(8))[0]
        kvc = struct.unpack("<Q", f.read(8))[0]
        kv = {}
        for _ in range(kvc):
            try:
                k = _rs(f); vt = struct.unpack("<I", f.read(4))[0]; v = _rv(f, vt)
                kv[k] = v
            except Exception as e:
                kv["__err"] = str(e); break
        arch = kv.get("general.architecture", "?")
        out = {
            "path": str(p), "size_mb": size_mb, "gguf_version": ver,
            "tensor_count": tc, "kv_count": kvc, "architecture": arch,
            "context_length":   kv.get(f"{arch}.context_length"),
            "embedding_length": kv.get(f"{arch}.embedding_length"),
            "block_count":      kv.get(f"{arch}.block_count"),
            "head_count":       kv.get(f"{arch}.attention.head_count"),
            "head_count_kv":    kv.get(f"{arch}.attention.head_count_kv"),
            "tokenizer_model":  kv.get("tokenizer.ggml.model"),
            "vocab_size":       len(kv.get("tokenizer.ggml.tokens", [])
                                    if isinstance(kv.get("tokenizer.ggml.tokens"), list) else []),
        }
        if metadata: out["_kv"] = {k: str(v)[:80] for k,v in kv.items()}
        if tensors:
            tlist = []
            for _ in range(tc):
                try:
                    name  = _rs(f)
                    nd    = struct.unpack("<I", f.read(4))[0]
                    dims  = [struct.unpack("<Q", f.read(8))[0] for _ in range(nd)]
                    qt    = struct.unpack("<I", f.read(4))[0]
                    off   = struct.unpack("<Q", f.read(8))[0]
                    tlist.append({"name": name, "dims": dims,
                                  "qtype": GGML_T.get(qt,f"unk_{qt}"), "offset": off})
                except Exception as e:
                    tlist.append({"error": str(e)}); break
            out["tensors"] = tlist
    return out

# â”€â”€ Stub detector â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def check_stub():
    if not LOADER.exists():
        print(f"LOADER: NOT FOUND ({LOADER})"); return
    src = LOADER.read_text(encoding="utf-8", errors="replace")
    hardcoded = bool(re.search(r'"qwen2"|"291"|tensors:\s*291', src))
    delegates = bool(re.search(r'GgufLoaderStd|loader_std|hacedle', src))
    gguf_read = bool(re.search(r'read_exact|BufReader|File::open', src))
    status = (
        "DELEGATES_OK"      if delegates else
        "INLINE_PARSE_OK"   if gguf_read and not hardcoded else
        "HARDCODED_STUB"    if hardcoded else
        "STUB_EMPTY"
    )
    print(f"loader.rs size:      {LOADER.stat().st_size} bytes")
    print(f"delegates_hacedle:   {delegates}")
    print(f"hardcoded_values:    {hardcoded}")
    print(f"inline_file_read:    {gguf_read}")
    print(f"verdict:             {status}")
    if status == "HARDCODED_STUB":
        print("ACTION: brain/master/src/provider/gguf/loader.rs MUST delegate to hacedle::GgufLoaderStd")

# â”€â”€ Main â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

def main():
    if "--stub-check" in sys.argv:
        check_stub(); return
    if len(sys.argv) < 2:
        print(__doc__); sys.exit(1)
    path      = sys.argv[1]
    show_t    = "--tensors"  in sys.argv
    show_m    = "--metadata" in sys.argv
    r = parse_gguf(path, tensors=show_t, metadata=show_m)
    if "error" in r:
        print(f"ERROR: {r['error']}"); sys.exit(1)
    for k, v in r.items():
        if k.startswith("_") or k == "tensors": continue
        print(f"{k:<22}: {v}")
    if show_m and "_kv" in r:
        print("\nmetadata_kv:")
        for k, v in r["_kv"].items():
            print(f"  {k}: {v}")
    if show_t and "tensors" in r:
        print(f"\ntensors ({len(r['tensors'])}):")
        for t in r["tensors"][:30]:
            if "error" in t: print(f"  ERR: {t['error']}"); break
            print(f"  {t['name']:<48} {t['qtype']:<8} {t['dims']}")
        if len(r["tensors"]) > 30:
            print(f"  ... {len(r['tensors'])} total")
    # Auto stub check
    print("\n--- stub check ---")
    check_stub()

if __name__ == "__main__":
    main()
