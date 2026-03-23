# Flint GPUI Adapter

Status: active
Updated: 2026-03-23

This crate is the canonical GPUI integration layer for Flint.

It owns the renderer-facing support matrix for Flint contracts across
primitives, composites, and workstation shells, and it maps shared contract
semantics onto GPUI-native rendering and theme resolution.

## Current Role

- bridge `flint-*` contract crates into GPUI
- declare supported contract-backed spec coverage for parity metadata
- provide theme and style mapping used by GPUI-native render paths

## Next Task

Keep this README aligned with the GPUI baseline artifacts whenever adapter
ownership or crate structure changes.
