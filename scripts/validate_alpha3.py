#!/usr/bin/env python3
"""
Alpha-3 Deep Validation Script - Audits presence vs runtime
"""

import os

def check_content(path, patterns):
    """Check if file contains expected patterns"""
    try:
        with open(path.replace("/", os.sep), 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            return all(p in content for p in patterns)
    except:
        return False

def main():
    print("=" * 60)
    print("Alpha-3 Deep Audit - Presence vs Runtime")
    print("=" * 60)
    
    # PRESENCE CHECKS
    presence = [
        ("rr/cli Brain family", "t:/hace/engine/rr/cli/src/lib.rs", 
         ["CommandFamily::Brain"]),
        ("AS_COMMON router", "t:/hace/engine/hace/cli/src/bin/hace.rs",
         ["brain", "handle_brain"]),
        ("Brain CLI lib", "t:/hace/engine/hace/brain/cli/src/lib.rs",
         ["BrainCmd"]),
        ("ReplayRecord", "t:/hace/engine/hace/brain/session/src/replay_record.rs",
         ["ReplayRecord", "turn_id"]),
        ("BrainSession", "t:/hace/engine/hace/brain/session/src/lib.rs",
         ["BrainSession", "history"]),
    ]
    
    print("\n[PRESENCE] Module Structure:")
    presence_passed = 0
    for name, path, patterns in presence:
        exists = os.path.exists(path.replace("/", os.sep))
        has_content = exists and check_content(path, patterns)
        status = "PASS" if has_content else "FAIL"
        print(f"  [{status}] {name}")
        if has_content:
            presence_passed += 1
    
    # RUNTIME CHECKS (stub detection)
    runtime_stubs = [
        ("GGUF metadata stub", "t:/hace/engine/hace/brain/master/src/provider/gguf/mod.rs",
         ["TODO", "stub"]),
        ("Tokenizer stub", "t:/hace/engine/hace/brain/cli/src/gguf/tokenizer.rs",
         ["TODO", "placeholder"]),
        ("Model verify stub", "t:/hace/engine/hace/cli/src/bin/hace.rs",
         ["291 tensors", "42 metadata"]),
    ]
    
    print("\n[RUNTIME] Stub Detection:")
    runtime_stubbed = 0
    for name, path, patterns in runtime_stubs:
        exists = os.path.exists(path.replace("/", os.sep))
        is_stub = exists and check_content(path, patterns)
        status = "STUB" if is_stub else "REAL" if exists else "MISSING"
        print(f"  [{status}] {name}")
        if is_stub:
            runtime_stubbed += 1
    
    # INFERENCE CHECKS (what's missing)
    inference_missing = [
        "TensorBackend abstraction",
        "Attention implementation",
        "RMSNorm implementation",
        "KV cache",
        "Golden replay comparison",
        "Real GGUF loading",
    ]
    
    print("\n[INFERENCE] Missing Components:")
    for item in inference_missing:
        print(f"  [MISSING] {item}")
    
    # SUMMARY
    print("\n" + "=" * 60)
    print(f"PRESENCE: {presence_passed}/{len(presence)} modules present")
    print(f"RUNTIME: {runtime_stubbed} stubs, {len(runtime_stubs) - runtime_stubbed} real")
    print(f"INFERENCE: {len(inference_missing)} components needed")
    score = int(presence_passed / len(presence) * 100)
    print(f"\nAlpha-3 Score: ~{score}% architecture, ~15% inference")
    print("=" * 60)

if __name__ == "__main__":
    main()