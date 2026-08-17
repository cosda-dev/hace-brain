#!/usr/bin/env python3
"""
Alpha-3 Brain Runtime Check Script
Validates components toward first_real_token goal

Usage: py run_alpha3_check.py
"""

import os
import sys
import json
from datetime import datetime

# Base path for the project - adjust for actual structure
# Script is at: t:\hace\engine\hace\brain\scripts\
# Working dir: t:\hace\engine\hace\
# Project root should be t:\hace\engine\hace\
import pathlib
PROJECT_ROOT = pathlib.Path(os.path.dirname(os.path.abspath(__file__))).parent.parent

def check_file_exists(path):
    """Check if file exists"""
    full = os.path.join(PROJECT_ROOT, path)
    # Normalize path separators
    full = os.path.normpath(full)
    exists = os.path.exists(full)
    status = "PASS" if exists else "MISSING"
    print(f"  [{status}] {path}")
    return exists

def main():
    print("=" * 60)
    print("Alpha-3 Brain Runtime Check - 2026-06-04")
    print(f"Project Root: {PROJECT_ROOT}")
    print("=" * 60)
    
    print("\n[1] GGUF Runtime Components:")
    gguf_components = [
        "fem/hacedle/src/x/loader/gguf/loader.rs",
        "fem/hacedle/src/x/loader/gguf/loader_std.rs",
        "fem/hacedle/src/x/loader/gguf/header.rs",
        "fem/hacedle/src/x/loader/gguf/tensor_projection.rs",
    ]
    gguf_scores = sum(check_file_exists(f) for f in gguf_components)
    print(f"  Score: {gguf_scores}/{len(gguf_components)}")
    
    print("\n[2] Q4K Dequant Components:")
    q4k_components = [
        "fem/hacedle/src/quant_view/q4k_tensor_view.rs",
        "fem/hacedle/src/quant_view/dequant_dispatcher.rs",
        "fem/hacedle/src/x/loader/dequant/q4_k.rs",
    ]
    q4k_scores = sum(check_file_exists(f) for f in q4k_components)
    print(f"  Score: {q4k_scores}/{len(q4k_components)}")
    
    print("\n[3] Transformer Components:")
    transformer_components = [
        "fem/hacedle/src/x/provider/candle/layer.rs",
        "fem/hacedle/src/x/provider/candle/inference.rs",
        "fem/hacedle/src/ops/rmsnorm.rs",
        "fem/hacedle/src/ops/attention.rs",
        "fem/hacedle/src/ops/rope.rs",
        "fem/hacedle/src/ops/silu.rs",
    ]
    transformer_scores = sum(check_file_exists(f) for f in transformer_components)
    print(f"  Score: {transformer_scores}/{len(transformer_components)}")
    
    print("\n[4] LRO Components:")
    lro_components = [
        "fem/hacedle/src/x/loader/lro/mod.rs",
        "fem/hacedle/src/x/loader/lro/stack.rs",
        "fem/hacedle/src/x/loader/lro/seal.rs",
        "fem/hacedle/src/x/loader/lro/parser.rs",
    ]
    lro_scores = sum(check_file_exists(f) for f in lro_components)
    print(f"  Score: {lro_scores}/{len(lro_components)}")
    
    print("\n[5] Bridge Components:")
    bridge_components = [
        "brain/runtime/src/bridges/hacedle.rs",
        "brain/runtime/src/bridges/hacetral.rs",
    ]
    bridge_scores = sum(check_file_exists(f) for f in bridge_components)
    print(f"  Score: {bridge_scores}/{len(bridge_components)}")
    
    total = gguf_scores + q4k_scores + transformer_scores + lro_scores + bridge_scores
    total_expected = len(gguf_components) + len(q4k_components) + len(transformer_components) + len(lro_components) + len(bridge_components)
    
    print("\n" + "=" * 60)
    print(f"Total: {total}/{total_expected} components exist")
    print("=" * 60)
    
    # Generate report
    report = {
        "timestamp": datetime.now().isoformat(),
        "components_found": total,
        "components_total": total_expected,
        "alpha3_ready": total == total_expected
    }
    
    report_path = os.path.join(PROJECT_ROOT, "brain", "reports", "alpha3_check.json")
    os.makedirs(os.path.dirname(report_path), exist_ok=True)
    with open(report_path, "w") as f:
        json.dump(report, f, indent=2)
    
    print(f"\nReport saved to: brain/reports/alpha3_check.json")

if __name__ == "__main__":
    main()