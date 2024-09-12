# - Imports
# Pre-made
import inquirer
import os
import time
import math
import sys
# Custom
from colorConvert import hex_to_rgb
from spreadPoints import spreadPoints
from objParser import load

def distBetweenTwoPoints(point1:tuple[float, float, float], point2:tuple[float, float, float]):
    x1, y1, z1 = point1
    x2, y2, z2 = point2
    return math.sqrt((x2 - x1)**2 + (y2 - y1)**2 + (z2 - z1)**2)

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
        sys.exit(0)
        
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
    edges = load(name)
    points = []
    
    if inquirer.list_input("Subdivide Edges", choices=["Yes", "No"]) == "Yes":
        lowest = float('inf')
        for edge in edges:
            dist = distBetweenTwoPoints(edge[0], edge[1])
            if dist < lowest:
                lowest = dist
                
        while True:
            try:
                particlesPerBlock = int(input(f"Particles per {lowest} blocks: "))
                if particlesPerBlock < 1:
                    raise ValueError
                break
            except ValueError:
                print("Please input a positive integer")
                
        spacing = lowest / (particlesPerBlock + 1)
        
        for edge in edges:
            for point in spreadPoints(spacing, edge[0], edge[1]):
                points.append(point)
                
    else:
        for edge in edges:
            points.append(edge[0])
            points.append(edge[1])
            
                
    
    for point in points:
        lines.append(f"particle dust{"{"}color:[{color[0]},{color[1]},{color[2]}], scale:{scaling}{"}"} {orientation}{round(point[0],3)} {orientation}{round(point[1],3)} {orientation}{round(point[2],3)}\n")
    
    lines = set(lines)
    
    with open(f"{name[0:-4]}.mcfunction", "w") as o:
        for command in lines:
            o.write(command)



if __name__ == "__main__":
    main()