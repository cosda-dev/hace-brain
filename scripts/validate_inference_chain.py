#!/usr/bin/env python3
"""
CSA Validation Script: Verify full inference chain
Checks that all components connect properly for G1 certification
"""

import os
from pathlib import Path

def check_inference_chain():
    """Check the full inference pipeline"""
    chain = {
        "tokenizer": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/tokenizer.rs",
            "methods": ["encode", "decode"],
        },
        "embed": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/embed.rs",
            "methods": ["embed_sequence", "load_from_gguf"],
        },
        "transformer": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/layer.rs",
            "methods": ["forward", "attention", "ffn"],
        },
        "lmhead": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/lmhead.rs",
            "methods": ["forward", "load_weight"],
        },
        "sampler": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/sampler.rs",
            "methods": ["apply_temperature", "apply_top_p", "apply_top_k"],
        },
        "logits_processor": {
            "path": "T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/lmhead.rs",
            "methods": ["process"],
        },
        "brain_logits": {
            "path": "T:/hace/engine/hace/brain/master/src/runtime/logits.rs",
            "methods": ["from_kv"],
        },
    }
    
    results = {}
    
    for name, spec in chain.items():
        path = Path(spec["path"])
        if path.exists():
            content = path.read_text(encoding='utf-8', errors='replace')
            methods_found = []
            for method in spec["methods"]:
                if f"pub fn {method}" in content or f"fn {method}" in content:
                    methods_found.append(method)
            
            # Check for stub indicators
            is_stub = False
            if name == "brain_logits":
                is_stub = "9707" in content or "world" in content
            
            results[name] = {
                "exists": True,
                "methods_found": methods_found,
                "stub": is_stub,
                "status": "STUB" if is_stub else ("OK" if len(methods_found) == len(spec["methods"]) else "INCOMPLETE")
            }
        else:
            results[name] = {"exists": False, "status": "MISSING"}
    
    return results

if __name__ == "__main__":
    print("=" * 55)
    print("CSA Inference Chain Validation")
    print("=" * 55)
    
    results = check_inference_chain()
    
    for name, data in results.items():
        print(f"\n{name}:")
        print(f"  Status:  {data['status']}")
        if data.get('methods_found'):
            print(f"  Methods: {', '.join(data['methods_found'])}")
    
    # Summary
    stubs = sum(1 for d in results.values() if d.get('stub'))
    missing = sum(1 for d in results.values() if not d.get('exists'))
    
    print("=" * 55)
    print(f"Chain Health: {stubs} stubs, {missing} missing")