#!/usr/bin/env python3
"""
Validate GGUF inference chain skeleton
"""

import os

def check_exists(path):
    return os.path.exists(path.replace("/", os.sep))

def main():
    print("=" * 60)
    print("Alpha-3 GGUF Skeleton Validation")
    print("=" * 60)
    
    ggu_parts = [
        ("G1 - loader", "brain/master/src/provider/gguf/loader.rs"),
        ("G2 - metadata", "brain/master/src/provider/gguf/metadata.rs"),
        ("G3 - tensor", "brain/master/src/provider/gguf/tensor.rs"),
        ("G4 - mmap", "brain/master/src/provider/gguf/mmap.rs"),
        ("G5 - tokenizer", "brain/master/src/tokenizer/mod.rs"),
        ("G6 - kv_cache", "brain/master/src/runtime/kv_cache.rs"),
        ("G7 - logits", "brain/master/src/runtime/logits.rs"),
        ("G8 - prefill", "brain/master/src/runtime/prefill.rs"),
    ]
    
    base = "t:/hace/engine/hace"
    passed = 0
    for name, path in ggu_parts:
        exists = check_exists(os.path.join(base, path))
        status = "PASS" if exists else "FAIL"
        print(f"  [{status}] {name}")
        if exists:
            passed += 1
    
    print(f"\nTotal: {passed}/{len(ggu_parts)} components present")
    print("=" * 60)

if __name__ == "__main__":
    main()