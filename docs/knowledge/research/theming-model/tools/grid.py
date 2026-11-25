# Labelled grid of captures: python3 tools/grid.py out.png scale cols "title" label1=path1 label2=path2 ...
import sys
from PIL import Image, ImageDraw, ImageFont
out, scale, cols, title, items = sys.argv[1], float(sys.argv[2]), int(sys.argv[3]), sys.argv[4], sys.argv[5:]
pairs = [i.split("=", 1) for i in items]
ims = [Image.open(p).convert("RGB") for _, p in pairs]
w, h = int(ims[0].width * scale), int(ims[0].height * scale)
rows = (len(ims) + cols - 1) // cols
try: font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 15); tfont = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", 18)
except Exception: font = tfont = ImageFont.load_default()
sheet = Image.new("RGB", (cols * (w + 10) + 10, rows * (h + 28) + 40), (128, 128, 128))
d = ImageDraw.Draw(sheet)
d.text((10, 10), title, fill=(0, 0, 0), font=tfont)
for i, ((label, _), im) in enumerate(zip(pairs, ims)):
    x, y = 10 + (i % cols) * (w + 10), 40 + (i // cols) * (h + 28)
    sheet.paste(im.resize((w, h), Image.LANCZOS), (x, y + 22))
    d.text((x, y + 3), label, fill=(0, 0, 0), font=font)
sheet.save(out)
print("wrote", out, sheet.size)
