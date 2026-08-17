# HACE Brain

## Overview
Brain Runtime for HACE - Edge LLM inference, LoRA adapters, KV cache fusion.

## Structure
```text
brain/
├── base/           # BrainKernel trait, BrainError, BrainRuntime
├── cli/            # Brain CLI (sub module of Hace CLI)
├── runtime/        # Zeus CE coordinator (routes BrainKernel)
├── master/         # Brain master orchestration
├── reports/        # Status reports, audits
├── milestones/     # Milestone tracking
├── tests/          # Integration tests
└── scripts/        # Build/debug scripts
```

## Commands
```bash
hace brain model list
hace brain run --model qwen2.5-0.5b.gguf --prompt "hello"
hace brain lora attach finance.lro
hace brain replay --golden block0.bin
```

## Status
- Architecture: 90%
- Runtime Truth: 50-60%
- Next: A3-R1 first_real_token