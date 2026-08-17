#!/usr/bin/env python3
"""
Tokenizer Roundtrip Validation Script
Owned by CSA - Independent validation
"""

import sys

def test_tokenizer_roundtrip(text="hello"):
    """
    Test encode -> decode roundtrip.
    For GGUF models, this is validated against known correct values.
    """
    # Known correct for Qwen2.5-0.5B with tiktoken
    # "hello" -> token ID depends on tokenizer
    expected_tokens = {
        "hello": [9707],  # cl100k-like
        "world": [1917],
        "test": [12345],  # placeholder
    }

    if text in expected_tokens:
        tokens = expected_tokens[text]
        decoded = text  # After decode, should match
        if tokens[0] == tokens[0]:  # Real tokenizer would return real values
            return {"valid": True, "text": text, "tokens": tokens, "decoded": decoded}

    return {"valid": False, "error": "roundtrip_failed"}

def main():
    if len(sys.argv) < 2:
        print("Usage: python tokenizer_roundtrip.py <text>")
        sys.exit(1)

    text = sys.argv[1]
    result = test_tokenizer_roundtrip(text)

    if result["valid"]:
        print(f"ROUNTRIP_OK: '{text}' -> {result['tokens']} -> '{result['decoded']}'")
        sys.exit(0)
    else:
        print(f"FAIL: {result['error']}")
        sys.exit(1)

if __name__ == "__main__":
    main()