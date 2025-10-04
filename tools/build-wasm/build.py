"""
This script builds the main application for the wasm32-unknown-unknown target
and bundles the wasm into an HTML file located at `target/last_stand.html`.
"""

import subprocess
import base64

target = "wasm32-unknown-unknown"
print(f"Building the game for {target}...")
result = subprocess.run([
    "cargo", "build",
    "--manifest-path", "../../Cargo.toml",
    "--target", target,
    "--release"
])
if result.returncode != 0:
    raise Exception("Cargo build failed!")

print("Constructing the HTML file...")
with open(f"../../target/{target}/release/last_stand.wasm", "rb") as file:
    wasm = base64.b64encode(file.read()).decode('utf-8')

with open("./templates/injector.js", "r") as file:
    injector = file.read()

with open("./templates/macroquad.js", "r") as file:
    macroquad = file.read()

with open("./templates/template.html", "r") as file:
    html = file.read()

result = html\
    .replace("INSERT_MACROQUAD_HERE", macroquad)\
    .replace("INSERT_INJECTOR_HERE", injector.replace("INSERT_WASM_HERE", wasm))

with open("../../target/last_stand.html", "w") as file:
    file.write(result)

print("HTML file successfully constructed.")
