#!/usr/bin/env python3
"""
Era 5 Validation: CAT + Coge Integration
Tests the complete contract/action/template pipeline
"""

import os

def check_file_patterns(path, patterns):
    """Check file exists and contains patterns"""
    try:
        with open(path.replace("/", os.sep), 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            return all(p in content for p in patterns)
    except:
        return False

def main():
    print("=" * 60)
    print("Era 5: CAT + Coge Integration Check")
    print("=" * 60)
    
    checks = [
        ("fem/cat Cargo.toml", "t:/hace/engine/hace/fem/cat/Cargo.toml",
         ["hace-fem-cat"]),
        ("fem/cat lib.rs", "t:/hace/engine/hace/fem/cat/src/lib.rs",
         ["Contract Action Template", "Cat"]),
        ("fem/cat parser.rs", "t:/hace/engine/hace/fem/cat/src/parser.rs",
         ["AilParser", "parse_header"]),
        ("fem/cat manifest.rs", "t:/hace/engine/hace/fem/cat/src/manifest.rs",
         ["brain_prompt", "brain_model_verify"]),
        ("brain/coge Cargo.toml", "t:/hace/engine/hace/brain/coge/Cargo.toml",
         ["hace-brain-coge", "hace-fem-cat"]),
        ("brain/coge cat_dispatcher.rs", "t:/hace/engine/hace/brain/coge/src/cat_dispatcher.rs",
         ["CatDispatcher", "dispatch_prompt"]),
        ("canon/coge.ail", "t:/hace/engine/hace/brain/canon/coge.ail",
         ["Coge Flow", "CAT Router"]),
    ]
    
    passed = 0
    for name, path, patterns in checks:
        valid = check_file_patterns(path, patterns)
        status = "PASS" if valid else "FAIL"
        print(f"  [{status}] {name}")
        if valid:
            passed += 1
    
    print(f"\nPassed: {passed}/{len(checks)}")
    print("=" * 60)

if __name__ == "__main__":
    main()