# - Imports
# Pre-made
import inquirer
import os
import time
# Custom
from colorConvert import hex_to_rgb

# - Main Function
def main():
    # - User Feedback
    print("Searching for .obj files...")
    # - Searching for .obj files for selection
    foundFiles = []
    for (root, _, files) in os.walk(".", topdown=True):
        for file in files:
            # Checking if its a .obj file
            if file.split(".")[-1] == "obj":
                foundFiles.append(f"{root}{os.sep}{file}")
    
    # Checking if any files were found
    if len(foundFiles) > 0:
        print("Files Found!")
    else:
        print("Could Not Find Files :(")
        time.sleep(0.5)
        print("Quitting...")
        time.sleep(0.5)
        quit()
        
    # - Getting user Selected File
    name = inquirer.list_input("Select a file",choices = foundFiles)
    # - Get scaling value
    while True:
        # Ensuring no breakages
        try:
            scaling = float(input("What scaling value would you like to use (The default blender cube is 4 blocks at scaling 1, set to 0.5 for it to be 1 block)? "))
            break
        except ValueError:
            print("Please only input numbers!")
    
    # - Coloration     
    colorType = inquirer.list_input("What color type do you wish to use to select a color?", choices=["Hex", "RGB"])
    color = []
    
    # - Hex support
    if colorType == "Hex":
        hexValues = ["1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]
        while True:
            try:
                code = input("Input your hex code: ").lower()
                if len(code) == 6:
                    for value in code:
                        if value not in hexValues:
                            raise ValueError("Incorrect Hex Code")
                    code = hex_to_rgb(code)
                    break
            except ValueError:
                print("Invalid Hex Code")
                
    else:
        code = input("Input your RGB value separated by commas: ")
        code.split(",")
        for i in code:
            i.lstrip()
            
    for value in code:
        color.append(value/255)
        
    orientation = inquirer.list_input("Choose your rotation space:", choices=["~", "^"])
            
    lines = []
    with open(name) as o:
        for line in o:
            coords = line.split()
            x = float(coords[1].strip()) * scaling
            y = float(coords[2].strip()) * scaling
            z = float(coords[3].strip()) * scaling
            lines.append(f"particle dust{"{"}color:[{color[0]},{color[1]},{color[2]}],scale:1{"}"} {orientation}{x} {orientation}{y} {orientation}{z} 0 0 0 0 1 force\n")
    
    with open(f"{name[0:-4]}.mcfunction", "w") as o:
        for command in lines:
            o.write(command)

if __name__ == "__main__":
    main()