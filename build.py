# import os

# os.system("trunk build")

# text = "";
# with open("dist/index.html", "r") as f:
#     text = f.read()

# text = text.replace("bindings from '/eframe_template.js'", "bindings from '/webcamera/eframe_template.js'")
# text = text.replace("const wasm = await init('/eframe_template_bg.wasm');", "const wasm = await init('/webcamera/eframe_template_bg.wasm');")
# text = text.replace("href=\"/eframe_template.js\"", "href=\"/webcamera/eframe_template.js\"")
# text = text.replace("href=\"/eframe_template_bg.wasm\"", "href=\"/webcamera/eframe_template_bg.wasm\"")

# with open("dist/index.html", "w") as f:
#     f.write(text)