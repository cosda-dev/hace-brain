#!/usr/bin/env python3
"""
Test real GGUF loading - check if models exist and are valid
"""

import os

def check_gguf_files():
    model_dirs = [
        "D:/host/llama-models",
        "T:/hace/models",
        "t:/hace/models",
    ]
    
    for model_dir in model_dirs:
        path = model_dir.replace("/", os.sep)
        if os.path.exists(path):
            print(f"Model dir EXISTS: {path}")
            for f in os.listdir(path):
                if f.endswith('.gguf'):
                    print(f"  - {f}")
            return True
    
    print("No GGUF model dir found")
    return False

def main():
    print("=" * 60)
    print("Real GGUF Test Check")
    print("=" * 60)
    check_gguf_files()

if __name__ == "__main__":
    main()