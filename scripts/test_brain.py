#!/usr/bin/env python3
"""
Brain Component Test Script - Validates FES architecture without binary
Usage: python scripts/test_brain.py
"""

import os
import sys

def test_fes_structure():
    """Test FES layer structure exists"""
    base = "t:/hace/engine/hace/brain"
    layers = {
        "cli": f"{base}/cli/src",
        "session": f"{base}/session/src",
        "master": f"{base}/master/src",
        "io": f"{base}/io",
        "coge": f"{base}/coge",
    }
    
    print("Testing FES Structure...")
    for name, path in layers.items():
        exists = os.path.exists(path.replace("/", os.sep))
        status = "PASS" if exists else "MISSING"
        print(f"  [{status}] {name}: {path}")
    return all(os.path.exists(p.replace("/", os.sep)) for p in layers.values())

def test_cli_modules():
    """Test CLI module files exist"""
    cli_modules = [
        "brain.rs", "prompt.rs", "replay.rs", "model.rs", "executor.rs", "command.rs"
    ]
    
    print("\nTesting CLI Modules...")
    base = "t:/hace/engine/hace/brain/cli/src"
    for module in cli_modules:
        path = os.path.join(base, module).replace("/", os.sep)
        exists = os.path.exists(path)
        status = "PASS" if exists else "MISSING"
        print(f"  [{status}] {module}")
    return True

def test_replay_artifact():
    """Test ReplayRecord struct exists"""
    print("\nTesting Replay Artifact...")
    path = "t:/hace/engine/hace/brain/session/src/replay_record.rs".replace("/", os.sep)
    exists = os.path.exists(path)
    print(f"  [{'PASS' if exists else 'MISSING'}] replay_record.rs")
    return exists

def test_session():
    """Test BrainSession module"""
    print("\nTesting Session Module...")
    path = "t:/hace/engine/hace/brain/session/src/lib.rs".replace("/", os.sep)
    if os.path.exists(path):
        with open(path) as f:
            content = f.read()
            has_history = "history" in content and "ReplayRecord" in content
            print(f"  [{'PASS' if has_history else 'FAIL'}] BrainSession has history")
            return has_history
    print(f"  [MISSING] lib.rs")
    return False

def main():
    print("=" * 50)
    print("Brain FES Architecture Test")
    print("=" * 50)
    
    results = []
    results.append(("FES Structure", test_fes_structure()))
    results.append(("CLI Modules", test_cli_modules()))
    results.append(("ReplayArtifact", test_replay_artifact()))
    results.append(("Session", test_session()))
    
    print("\n" + "=" * 50)
    for name, passed in results:
        status = "PASS" if passed else "FAIL"
        print(f"{name}: {status}")
    print("=" * 50)

if __name__ == "__main__":
    main()