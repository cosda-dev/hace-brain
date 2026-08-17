#!/usr/bin/env python3
"""
CSA Validation Script: Verify weight loading from GGUF tensors
Checks that LMHead, EmbedEngine, TransformerLayer load real weights
"""

import os
from pathlib import Path

def check_weight_initialization():
    """Check if inference components are stubbed or real"""
    paths = {
        "lmhead": Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/lmhead.rs"),
        "embed": Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/embed.rs"),
        "layer": Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/layer.rs"),
        "inference": Path("T:/hace/engine/hace/fem/hacedle/src/x/provider/candle/inference.rs"),
    }
    
    results = {}
    
    # Check LMHead
    if paths["lmhead"].exists():
        content = paths["lmhead"].read_text(encoding='utf-8', errors='replace')
        empty_weight = "weight: Vec::new()" in content
        has_load_weight = "load_weight" in content or "from_gguf" in content
        results["lmhead"] = {
            "exists": True,
            "empty_weight": empty_weight,
            "has_load_method": has_load_weight,
            "status": "STUB" if empty_weight else "OK"
        }
    
    # Check EmbedEngine
    if paths["embed"].exists():
        content = paths["embed"].read_text(encoding='utf-8', errors='replace')
        empty_embed = "embeddings: Vec::new()" in content
        has_load = "load_from_gguf" in content
        results["embed"] = {
            "exists": True,
            "empty_embeddings": empty_embed,
            "has_load_method": has_load,
            "status": "STUB" if empty_embed else "OK"
        }
    
    # Check Layer
    if paths["layer"].exists():
        content = paths["layer"].read_text(encoding='utf-8', errors='replace')
        has_weights_param = "_attention_weights" in content or "_ffn_weights" in content
        placeholder_forward = "hidden_states.to_vec()" in content
        results["layer"] = {
            "exists": True,
            "takes_weights": has_weights_param,
            "placeholder_forward": placeholder_forward,
            "status": "PLANE" if placeholder_forward else "OK"
        }
    
    # Check Inference
    if paths["inference"].exists():
        content = paths["inference"].read_text(encoding='utf-8', errors='replace')
        loads_weights = "load_weight" in content or "from_tensor" in content
        results["inference"] = {
            "exists": True,
            "loads_weights": loads_weights,
            "status": "STUB" if not loads_weights else "OK"
        }
    
    return results

if __name__ == "__main__":
    print("=" * 50)
    print("CSA Weight Loading Check")
    print("=" * 50)
    
    results = check_weight_initialization()
    
    total_stub = 0
    for component, data in results.items():
        if data["status"] in ("STUB", "PLANE"):
            total_stub += 1
        print(f"{component}: {data['status']}")
        if data.get("empty_weight") or data.get("empty_embeddings"):
            print(f"  → Empty storage detected!")
    
    print("=" * 50)
    print(f"WEIGHT STATUS: {total_stub} components still stubbed")