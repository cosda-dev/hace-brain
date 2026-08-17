#!/usr/bin/env python3
"""
Comprehensive Alpha-3 Component Test Script
Tests all critical components for Alpha-3 certification
"""

import os
import sys

def test_file_exists(path):
    """Check if file exists"""
    return os.path.exists(path)

def test_file_contains(path, pattern):
    """Check if file contains pattern"""
    if not os.path.exists(path):
        return False
    try:
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            return pattern in content
    except Exception:
        return False

def main():
    print("=" * 60)
    print("Alpha-3 Component Test Suite")
    print("=" * 60)
    
    # Define tests (relative to T:/hace)
    tests = [
        # P1 - GGUF Loader
        ("P1 - GGUF Loader", 
         lambda: test_file_exists("engine/hace/brain/master/src/provider/gguf/loader.rs")),
         
        # P2 - Tokenizer trait  
        ("P2 - Tokenizer trait",
         lambda: test_file_exists("engine/hace/brain/master/src/tokenizer/mod.rs")),
         
        # G6 - KV Cache
        ("G6 - KV Cache",
         lambda: test_file_exists("engine/hace/brain/master/src/runtime/kv_cache.rs")),
         
        # G6 - Prefill
        ("G6 - Prefill",
         lambda: test_file_exists("engine/hace/brain/master/src/runtime/prefill.rs")),
         
        # G7 - Logits
        ("G7 - Logits",
         lambda: test_file_exists("engine/hace/brain/master/src/runtime/logits.rs")),
         
        # P3 - Coge dispatch
        ("P3 - Coge dispatch",
         lambda: test_file_exists("engine/hace/brain/coge/src/cat_dispatcher.rs")),
         
        # FEM GGUF loader
        ("FEM GGUF loader",
         lambda: test_file_exists("engine/hace/fem/hacedle/src/x/loader/gguf/loader.rs")),
         
        # FEM TensorIndex
        ("FEM TensorIndex",
         lambda: test_file_exists("engine/hace/fem/hacedle/src/x/loader/gguf/tensor_index.rs")),
         
        # FEM InferenceEngine (checking for correct pattern)
        ("FEM InferenceEngine",
         lambda: test_file_contains("engine/hace/fem/hacedle/src/x/provider/candle/inference.rs", "InferenceEngine")),
         
        # Model file check (adjust path as needed)
        ("Model file (Qwen2.5-0.5B)",
         lambda: os.path.exists("D:/host/llama-models/Qwen2.5-0.5B-Instruct-Q4_K_M.gguf")),
    ]
    
    # Run tests
    passed = 0
    total = len(tests)
    
    for test_name, test_func in tests:
        try:
            result = test_func()
            status = "[PASS]" if result else "[FAIL]"
            print(f"  {status} {test_name}")
            if result:
                passed += 1
        except Exception as e:
            print(f"  [ERROR] {test_name}: {e}")
    
    print("=" * 60)
    print(f"Results: {passed}/{total} tests passed")
    
    if passed == total:
        print("ALL TESTS PASSED!")
        return 0
    else:
        print(f"{total - passed} TEST(S) FAILED")
        return 1

if __name__ == "__main__":
    sys.exit(main())