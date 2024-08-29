# - Imports
# Pre-made
import inquirer
import os
import time
import math
# Custom
from colorConvert import hex_to_rgb

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
    vertexes = []
    edges = []
    with open(name) as o:
        lines.append("# Verticies\n")
        for line in o:
            coords = line.split()
            if coords[0] == "v":
                x = float(coords[1].strip()) * scaling
                y = float(coords[2].strip()) * scaling
                z = float(coords[3].strip()) * scaling
                vertexes.append((x, y, z))
                lines.append(f"particle dust{"{"}color:[{color[0]},{color[1]},{color[2]}],scale:1{"}"} {orientation}{x} {orientation}{y} {orientation}{z} 0 0 0 0 1 force\n")
            elif coords[0] == "f":
                edgeVertexes = []
                for coord in coords:
                    if coord == "f":
                        continue
                    edgeVertexes.append(int(coord.split("/")[0]))
                for vertexA in edgeVertexes:
        
                   edges.append((vertexes[vertexA-1], vertexes[edgeVertexes[edgeVertexes.index(vertexA)-1]-1]))
        if inquirer.list_input("Subdivide edges", choices=["Yes", "No"]) == "Yes":
            lines.append("# Edges\n")
            lowest:int | bool = False
            divisions = 0
            for edge in edges:
                current = distBetweenTwoPoints(*edge)
                if not lowest:
                    lowest = current
                elif current < lowest:
                    lowest = current
            while True:
                try:
                    divisions = int(input(f"Particles per {lowest} blocks: "))
                    break
                except ValueError:
                    print("Input an integer value please")
            
            edgePoints = []
            divisions += 1
            spacing = int(round(lowest / divisions) + 1)
            for edge in edges:
                x = 0
                y = 0
                z = 0
                t = spacing / (divisions - 1)
                for _ in range(divisions):
                    x = edge[0][0] + t * (edge[1][0] - edge[0][0])
                    y = edge[0][1] + t * (edge[1][1] - edge[0][1])
                    z = edge[0][2] + t * (edge[1][2] - edge[0][2])
                    edgePoints.append((x,y,z))
            for point in edgePoints:
                x = float(point[0])
                y = float(point[1])
                z = float(point[2])
                lines.append(f"particle dust{"{"}color:[{color[0]},{color[1]},{color[2]}],scale:1{"}"} {orientation}{x} {orientation}{y} {orientation}{z} 0 0 0 0 1 force\n")
        
                
    
    with open(f"{name[0:-4]}.mcfunction", "w") as o:
        for command in lines:
            o.write(command)



if __name__ == "__main__":
    main()