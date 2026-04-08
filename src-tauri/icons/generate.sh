file=source.svg

inkscape $file --export-type=png --export-filename='32x32.png'   -w 32  -h 32
inkscape $file --export-type=png --export-filename='128x128.png' -w 128 -h 128
inkscape $file --export-type=png --export-filename='128x128@2x.png' -w 256 -h 256
inkscape $file --export-type=png --export-filename='icon.png' -w 512 -h 512

