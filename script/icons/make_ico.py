#!/usr/bin/env python3
"""Pack PNGs into a multi-resolution .ico.

Windows has accepted PNG-compressed ICO entries since Vista, so each size is
embedded verbatim rather than re-encoded as a BMP.
"""

import struct
import sys
from pathlib import Path

SIZES = [16, 32, 48, 64, 128, 256]


def main() -> int:
    src_dir = Path(sys.argv[1])
    out_path = Path(sys.argv[2])

    images = []
    for size in SIZES:
        data = (src_dir / f"ico-{size}.png").read_bytes()
        images.append((size, data))

    header = struct.pack("<HHH", 0, 1, len(images))
    offset = len(header) + 16 * len(images)

    entries = bytearray()
    payload = bytearray()
    for size, data in images:
        # 256 is encoded as 0 in the directory entry.
        dim = 0 if size >= 256 else size
        entries += struct.pack(
            "<BBBBHHII", dim, dim, 0, 0, 1, 32, len(data), offset
        )
        payload += data
        offset += len(data)

    out_path.write_bytes(header + bytes(entries) + bytes(payload))
    print(f"wrote {out_path} ({out_path.stat().st_size} bytes, {len(images)} sizes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
