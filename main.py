import inquirer
from colorConvert import hex_to_rgb


def main():
    while True:
        try:
            name = input("What is the name of your obj file (don't include the .obj)? ")
            f = open(f"{name}.obj")
            f.close()
            break
        except FileNotFoundError:
            print("That file does not exist!")
    while True:
        try:
            scaling = float(input("What scaling value would you like to use (The default blender cube is 4 blocks at scaling 1, set to 0.5 for it to be 1 block)? "))
            break
        except ValueError:
            print("Please only input numbers!")
            
    colorType = inquirer.list_input("What color type do you wish to use to select a color?", choices=["Hex", "RGB"])
    color = []
    
    if colorType == "Hex":
        hexValues = ["1","2","3","4","5","6","7","8","9","a","b","c","d","e","f"]
        while True:
            try:
                code = input("Input your hex code: ")
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
            
    lines = []
    with open(f"{name}.obj") as o:
        for line in o:
            coords = line.split()
            x = float(coords[1].strip()) * scaling
            y = float(coords[2].strip()) * scaling
            z = float(coords[3].strip()) * scaling
            lines.append(f"particle dust {color[0]} {color[1]} {color[2]} 1 ~{x} ~{y} ~{z} 0 0 0 0 1 force\n")
    
    with open(f"{name}.mcfunction", "w") as o:
        for command in lines:
            o.write(command)

if __name__ == "__main__":
    main()