#!/usr/bin/env python3
"""
CSA Alpha-3 GGUF Verify Tool
Kiem tra GGUF file thuc su: magic, version, tensor count, metadata KV count.
So sanh voi brain/master stub output (hardcoded 291 tensors vs thuc te).

Usage:
  python3 gguf_verify.py <model.gguf>
  python3 gguf_verify.py <model.gguf> --tensors   # list tensor names
  python3 gguf_verify.py <model.gguf> --metadata  # list KV pairs
"""

import sys, struct, json
from pathlib import Path

GGUF_MAGIC = b"GGUF"

# GGML quant type names
GGML_TYPE = {
    0: "F32", 1: "F16", 2: "Q4_0", 3: "Q4_1",
    6: "Q5_0", 7: "Q5_1", 8: "Q8_0", 9: "Q8_1",
    10: "Q2_K", 11: "Q3_K", 12: "Q4_K", 13: "Q5_K",
    14: "Q6_K", 15: "Q8_K", 19: "IQ4_NL", 20: "IQ4_XS",
    29: "BF16",
}

# GGUF metadata value types
GGUF_VAL_TYPE = {
    0: "uint8", 1: "int8", 2: "uint16", 3: "int16",
    4: "uint32", 5: "int32", 6: "float32",
    7: "bool", 8: "string", 9: "array",
    10: "uint64", 11: "int64", 12: "float64",
}

def read_string(f) -> str:
    n = struct.unpack("<Q", f.read(8))[0]
    return f.read(n).decode("utf-8", errors="replace")

def read_value(f, vtype: int):
    if vtype == 0:  return struct.unpack("<B", f.read(1))[0]
    if vtype == 1:  return struct.unpack("<b", f.read(1))[0]
    if vtype == 2:  return struct.unpack("<H", f.read(2))[0]
    if vtype == 3:  return struct.unpack("<h", f.read(2))[0]
    if vtype == 4:  return struct.unpack("<I", f.read(4))[0]
    if vtype == 5:  return struct.unpack("<i", f.read(4))[0]
    if vtype == 6:  return struct.unpack("<f", f.read(4))[0]
    if vtype == 7:  return bool(struct.unpack("<B", f.read(1))[0])
    if vtype == 8:  return read_string(f)
    if vtype == 10: return struct.unpack("<Q", f.read(8))[0]
    if vtype == 11: return struct.unpack("<q", f.read(8))[0]
    if vtype == 12: return struct.unpack("<d", f.read(8))[0]
    if vtype == 9:  # array
        elem_type = struct.unpack("<I", f.read(4))[0]
        count     = struct.unpack("<Q", f.read(8))[0]
        # For large arrays, only sample first 3
        sample = []
        for i in range(count):
            v = read_value(f, elem_type)
            if i < 3:
                sample.append(v)
        if count > 3:
            sample.append(f"... ({count} total)")
        return sample
    return None  # unknown type, caller must handle

def parse_gguf(path: str, show_tensors: bool = False, show_metadata: bool = False) -> dict:
    p = Path(path)
    if not p.exists():
        return {"error": f"file not found: {path}"}

    result = {"path": str(p), "size_mb": round(p.stat().st_size / 1_048_576, 2)}

    with open(p, "rb") as f:
        # Header
        magic = f.read(4)
        if magic != GGUF_MAGIC:
            return {**result, "error": f"not GGUF â€” magic={magic.hex()}"}

        version      = struct.unpack("<I", f.read(4))[0]
        tensor_count = struct.unpack("<Q", f.read(8))[0]
        kv_count     = struct.unpack("<Q", f.read(8))[0]

        result.update({
            "gguf_version": version,
            "tensor_count": tensor_count,
            "kv_count":     kv_count,
        })

        # Metadata KV
        metadata = {}
        for _ in range(kv_count):
            try:
                key    = read_string(f)
                vtype  = struct.unpack("<I", f.read(4))[0]
                value  = read_value(f, vtype)
                metadata[key] = value
            except Exception as e:
                metadata["__parse_error"] = str(e)
                break

        # Extract canonical fields
        arch = metadata.get("general.architecture", "unknown")
        result["architecture"]    = arch
        result["context_length"]  = metadata.get(f"{arch}.context_length", metadata.get("llm.context_length"))
        result["embedding_length"]= metadata.get(f"{arch}.embedding_length")
        result["block_count"]     = metadata.get(f"{arch}.block_count")
        result["head_count"]      = metadata.get(f"{arch}.attention.head_count")
        result["head_count_kv"]   = metadata.get(f"{arch}.attention.head_count_kv")
        result["vocab_size"]      = metadata.get("tokenizer.ggml.tokens", [])
        result["vocab_size"]      = len(result["vocab_size"]) if isinstance(result["vocab_size"], list) else None
        result["tokenizer_model"] = metadata.get("tokenizer.ggml.model")

        if show_metadata:
            result["metadata_kv"] = {
                k: str(v)[:80] for k, v in metadata.items()
            }

        # Tensor index
        if show_tensors:
            tensors = []
            for _ in range(tensor_count):
                try:
                    name  = read_string(f)
                    ndims = struct.unpack("<I", f.read(4))[0]
                    dims  = [struct.unpack("<Q", f.read(8))[0] for _ in range(ndims)]
                    qtype = struct.unpack("<I", f.read(4))[0]
                    offset= struct.unpack("<Q", f.read(8))[0]
                    tensors.append({
                        "name":   name,
                        "dims":   dims,
                        "qtype":  GGML_TYPE.get(qtype, f"unk_{qtype}"),
                        "offset": offset,
                    })
                except Exception as e:
                    tensors.append({"error": str(e)})
                    break
            result["tensors"] = tensors

    return result

def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    path         = sys.argv[1]
    show_tensors = "--tensors"  in sys.argv
    show_meta    = "--metadata" in sys.argv

    r = parse_gguf(path, show_tensors=show_tensors, show_metadata=show_meta)

    if "error" in r:
        print(f"ERROR: {r['error']}")
        sys.exit(1)

    # YAML-style output (matches hace brain model inspect expected format)
    print(f"path:             {r['path']}")
    print(f"size_mb:          {r['size_mb']}")
    print(f"gguf_version:     {r['gguf_version']}")
    print(f"tensor_count:     {r['tensor_count']}")
    print(f"kv_count:         {r['kv_count']}")
    print(f"architecture:     {r['architecture']}")
    print(f"context_length:   {r['context_length']}")
    print(f"embedding_length: {r['embedding_length']}")
    print(f"block_count:      {r['block_count']}")
    print(f"head_count:       {r['head_count']}")
    print(f"head_count_kv:    {r['head_count_kv']}")
    print(f"vocab_size:       {r.get('vocab_size', 'n/a')}")
    print(f"tokenizer_model:  {r.get('tokenizer_model', 'n/a')}")

    if show_meta and "metadata_kv" in r:
        print("\nmetadata_kv:")
        for k, v in r["metadata_kv"].items():
            print(f"  {k}: {v}")

    if show_tensors and "tensors" in r:
        print(f"\ntensors ({len(r['tensors'])}):")
        for t in r["tensors"][:20]:
            if "error" in t:
                print(f"  ERROR: {t['error']}")
                break
            print(f"  {t['name']:<50} {t['qtype']:<8} {t['dims']}")
        if len(r["tensors"]) > 20:
            print(f"  ... ({len(r['tensors'])} total, use --tensors to see all)")

    # CSA comparison: detect if brain/master loader would output correct data
    print("\n--- CSA Stub Comparison ---")
    stub_correct = (r["tensor_count"] == 291 and r["architecture"] == "qwen2")
    if stub_correct:
        print("NOTE: This model matches hardcoded stub values (Qwen2 291t).")
        print("      brain/master stub would APPEAR correct for this model only.")
    else:
        print(f"MISMATCH: stub hardcodes arch=qwen2/tensors=291,")
        print(f"          actual arch={r['architecture']}, tensors={r['tensor_count']}")
        print("          brain/master loader.rs MUST delegate to real parser.")

if __name__ == "__main__":
    main()
