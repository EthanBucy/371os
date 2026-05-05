from io import BytesIO
from pathlib import Path
import sys

import numpy as np
import requests
from PIL import Image

IMG_URL = "https://cd-public.github.io/ai101/images/photo-cat.jpg"
DUMP_NAME = "dump.ppm"
OUT = Path("src/colors/img.rs")
ROWS = 25
COLS = 80

if len(sys.argv) > 1:
    IMG_URL = sys.argv[1]


def load_vga_colors():
    if Path(DUMP_NAME).exists():
        img = np.array(Image.open(DUMP_NAME).convert("RGB"))
        return img[0][::45][:16].astype(np.int32)

    return np.array(
        [
            [0, 0, 0],
            [0, 0, 168],
            [0, 168, 0],
            [0, 168, 168],
            [168, 0, 0],
            [168, 0, 168],
            [168, 87, 0],
            [168, 168, 168],
            [87, 87, 87],
            [87, 87, 255],
            [87, 255, 87],
            [87, 255, 255],
            [255, 87, 87],
            [255, 87, 255],
            [255, 255, 87],
            [255, 255, 255],
        ],
        dtype=np.int32,
    )


def nearest_color(pixel, colors):
    diff = colors - pixel.astype(np.int32)
    dist = np.sum(diff * diff, axis=1)
    return int(np.argmin(dist))


colors = load_vga_colors()

response = requests.get(IMG_URL, timeout=20)
response.raise_for_status()

img = Image.open(BytesIO(response.content)).convert("RGB")
img = img.resize((COLS, ROWS))
arr = np.array(img)

mapped = []
for row in range(ROWS):
    for col in range(COLS):
        mapped.append(nearest_color(arr[row, col], colors) << 4)

OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text(f"pub const ARR: [u8; {ROWS * COLS}] = {mapped};\n")
print(f"wrote {OUT} with {ROWS * COLS} entries")
