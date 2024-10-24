import os

os.system("trunk build")

text = "";
with open("dist/index.html", "r") as f:
    text = f.read()

text = text.replace("bindings from '/eframe_template.js'", "bindings from './eframe_template.js'")

with open("dist/index.html", "w") as f:
    f.write(text)