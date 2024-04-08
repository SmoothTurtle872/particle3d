def hex_to_rgb(code) -> tuple[int]:
    rgb = []
    for i in (0, 2, 4):
        decimal = int(code[i:i+2], 16)
        rgb.append(decimal)
        
    return tuple(rgb)