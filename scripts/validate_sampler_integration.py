#!/usr/bin/env python3
"""
CSA Validation Script: Verify sampler integration
Checks that LogitsProcessor is properly used in inference chain
"""

import os
from pathlib import Path

def check_logits_processor():
    """Check if logits processor exists and is used"""
    inference_path = Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/inference.rs")
    sampler_path = Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/sampler.rs")
    
    results = {}
    
    # Check inference.rs
    if inference_path.exists():
        content = inference_path.read_text(encoding='utf-8', errors='replace')
        uses_processor = "logits_processor" in content
        has_process = "process(" in content
        results["inference"] = {
            "exists": True,
            "uses_logits_processor": uses_processor,
            "has_process_call": has_process,
            "status": "OK" if uses_processor else "MISSING_PROCESSOR"
        }
    else:
        results["inference"] = {"exists": False, "status": "MISSING"}
    
    # Check sampler.rs
    if sampler_path.exists():
        content = sampler_path.read_text(encoding='utf-8', errors='replace')
        has_temperature = "apply_temperature" in content
        has_top_p = "apply_top_p" in content or "top_p" in content
        has_top_k = "top_k" in content
        has_process = "pub fn process(" in content
        
        results["sampler"] = {
            "exists": True,
            "has_temperature": has_temperature,
            "has_top_p": has_top_p,
            "has_top_k": has_top_k,
            "has_process": has_process,
            "status": "OK" if all([has_temperature, has_top_p, has_top_k, has_process]) else "INCOMPLETE"
        }
    else:
        results["sampler"] = {"exists": False, "status": "MISSING"}
    
    # Check brain/master logits.rs
    logits_path = Path("T:/hace/engine/hace/brain/master/src/runtime/logits.rs")
    if logits_path.exists():
        content = logits_path.read_text(encoding='utf-8', errors='replace')
        uses_kv = "kv_cache" in content or "KvCache" in content
        returns_hardcoded = "9707" in content or "TOPK" in content
        results["brain_logits"] = {
            "exists": True,
            "uses_kv_cache": uses_kv,
            "returns_hardcoded": returns_hardcoded,
            "status": "STUB" if returns_hardcoded else ("OK" if uses_kv else "INCOMPLETE")
        }
    else:
        results["brain_logits"] = {"exists": False, "status": "MISSING"}
    
    return results

if __name__ == "__main__":
    print("=" * 50)
    print("CSA Sampler Integration Check")
    print("=" * 50)
    
    results = check_logits_processor()
    
    for component, data in results.items():
        print(f"\n{component}:")
        print(f"  Status: {data['status']}")
        for key, value in data.items():
            if key not in ('exists', 'status'):
                print(f"  {key}: {'✅' if value else '❌'}")
    
    # Overall health
    hacedle_ok = results.get("inference", {}).get("status") == "OK"
    sampler_ok = results.get("sampler", {}).get("status") == "OK"
    brain_stub = results.get("brain_logits", {}).get("returns_hardcoded", False)
    
    print("=" * 50)
    overall = "✅ SAMPLER READY" if hacedle_ok and sampler_ok else "⚠️ SAMPLER GAP"