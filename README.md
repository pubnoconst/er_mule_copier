# Elden Ring Mule Copier (CLI)

A command-line tool to copy character slots between Elden Ring save files (`.sl2`).

This tool performs deterministic, byte-level manipulation of Elden Ring save files. It copies fixed-size character slot data blobs and their corresponding headers at known offsets. It does not interpret game logic, stats, or inventory.

Back up your saves before use. This tool creates backups automatically, but you remain responsible for your data.

---

## Why this version is written in Go (and why the GUI was dropped)

This project was originally written in Rust and included a GUI.

It was rewritten in **Go** and reduced to **CLI-only** for a single, concrete reason:

**The lack of a stable ABI in C-based Linux GUI libraries, specifically `webkit-gtk` and `libsoup`.**

These libraries repeatedly broke binary compatibility across:
* Linux distributions
* distro upgrades
* Flatpak vs system installs
* minor version changes

Despite correct builds, the resulting binaries would fail at runtime due to:
* missing symbols
* incompatible shared library versions
* forced rebuilds tied to distro state

This made long-term distribution and maintenance impractical.

As a result:
* The GUI was abandoned
* Dynamic linking was rejected
* Static linking became a hard requirement

Go was chosen because it reliably produces **self-contained, statically linked binaries** without depending on:
* `glibc` ABI stability beyond a minimal baseline
* system-installed GUI stacks
* C-based user-space libraries with weak compatibility guarantees

This rewrite is not about language preference.  
It is about **avoiding ABI fragility in C-based Linux GUI ecosystems**.

---

## How it works (high level)

Elden Ring save files contain:

* Global save metadata
* A contiguous table of fixed-size **character headers**
* A separate region of fixed-size **character slot data blobs**

Key properties:

* Headers and slot data are not interleaved
* Slots are identified purely by index
* Inactive characters still have data; visibility is controlled by a flag
* Offsets and sizes are fixed and version-sensitive

The tool:

1. Loads the entire source and target save files into memory
2. Displays source and target character slots
3. Prompts for a source slot and a target slot
4. Copies:
   * the source slot data blob
   * the source slot header
5. Rewrites Steam ID references and checksums
6. Writes the updated target save back to disk

All operations are deterministic, bounded, and index-based.

---

## Usage

```bash
er_mule_copier -from path/to/source.sl2 -to path/to/target.sl2
