#!/usr/bin/env python3
"""
CSA Validation Script: Verify loader delegation pattern
Checks that brain/master/loader.rs calls hacedle methods correctly
"""

import os
import re
from pathlib import Path

def check_loader_delegation():
    """Check loader.rs for correct delegation pattern"""
    loader_path = Path("T:/hace/engine/hace/brain/master/src/provider/gguf/loader.rs")
    
    if not loader_path.exists():
        return {"status": "MISSING", "path": str(loader_path)}
    
    content = loader_path.read_text(encoding='utf-8', errors='replace')
    
    issues = []
    
    # Check for incorrect load_std call
    if "GgufLoader::load_std(" in content:
        issues.append({
            "type": "WRONG_METHOD",
            "line": None,
            "message": "Uses GgufLoader::load_std() which does not exist - should use GgufLoader::load()"
        })
    
    # Check for correct delegation
    delegates = "hacedle" in content or "GgufLoader::load(" in content or "ModelSpec" in content
    
    # Check for hardcoded architecture
    hardcoded = '"qwen2"' in content or '"llama"' in content and "metadata" not in content.lower()
    
    return {
        "status": "OK" if delegates and not hardcoded else "ISSUE",
        "delegates_to_hacedle": delegates,
        "hardcoded_arch": hardcoded,
        "issues": issues,
        "verdict": "DELEGATES_OK" if delegates else "NO_DELEGATION",
    }

def check_hacedle_loader():
    """Verify hacedle loader has load() method"""
    loader_path = Path("T:/hace/engine/hace/fem/hacedle/src/x/loader/gguf/loader.rs")
    
    if not loader_path.exists():
        return {"status": "MISSING", "path": str(loader_path)}
    
    content = loader_path.read_text(encoding='utf-8', errors='replace')
    
    has_load = "pub fn load(" in content
    has_load_std = "pub fn load_std(" in content
    
    return {
        "status": "OK",
        "has_load": has_load,
        "has_load_std": has_load_std,
        "verdict": "HAS_LOAD_METHOD" if has_load else "MISSING_LOAD"
    }

if __name__ == "__main__":
    print("=" * 50)
    print("CSA Loader Delegation Check")
    print("=" * 50)
    
    brain_result = check_loader_delegation()
    hacedle_result = check_hacedle_loader()
    
    print(f"brain/master/loader.rs: {brain_result['verdict']}")
    print(f"hacedle/loader.rs:      {hacedle_result['verdict']}")
    
    if brain_result.get('issues'):
        for issue in brain_result['issues']:
            print(f"  ISSUE: {issue['message']}")
    
    all_ok = (
        brain_result['status'] == 'OK' and 
        hacedle_result['status'] == 'OK'
    )
    
    print("=" * 50)
    print(f"DELEGATION HEALTH: {'✅ OK' if all_ok else '⚠️ ISSUE DETECTED'}")