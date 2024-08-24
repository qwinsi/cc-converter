# create the icon file of the project.

# This script requires the Python Imaging Library (PIL) to be installed.
# If you don't have PIL, you can install it with pip:
# pip install pillow

from PIL import Image, ImageDraw, ImageFont

# Define the icon size and colors
icon_size = (128, 128)
background_color = (0x1d, 0x6c, 0x86)  # Blue
text_color = (255, 255, 255)  # White

# Create a new image with the background color
icon = Image.new('RGB', icon_size, background_color)
# Create a drawing context
draw = ImageDraw.Draw(icon)
# Load a font file
font = ImageFont.truetype('arial.ttf', 80)
# Draw text
draw.text((8, 25), 'CC', fill=text_color, font=font)
# Save the icon file
icon.save('icon.png', 'PNG')
icon.save('icon.ico', 'ICO')
icon.save('icon.icns', 'ICNS')
