import caer
import os 
from pathlib import Path 
import shutil as sh

FOLDER = "@amorphous-soroban-client"
DEST_FOLDER = os.path.join(os.getcwd(), FOLDER)
files = caer.path.listdir(DEST_FOLDER, use_fullpath=True, recursive=True)

for file in files:
    if "dist/esm/invoke.js" in file:
        pathFile = Path(file)

        lines = pathFile.read_text()
        lines = lines.replace("wallet.", "wallet.default.")
        pathFile.write_text(lines)

    
destNodeModules = os.path.join(os.getcwd(), "node_modules", FOLDER)
if os.path.isdir(destNodeModules):  sh.rmtree(destNodeModules)

os.system(f"cp -r ./{FOLDER} {destNodeModules}")
sh.rmtree(DEST_FOLDER)

print("Done!")