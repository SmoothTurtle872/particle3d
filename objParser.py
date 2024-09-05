def load(path):
    with open(path) as f:
        obj = f.read().split("\n")
        
    vertexes = [line for line in obj if line.split(" ")[0] == "v"]
    faces = [line for line in obj if line.split(" ")[0] == "f"]
    for vertex in vertexes:
        vertexes[vertexes.index(vertex)] = vertex.split(" ")[1:]
    for face in faces:
        faces[faces.index(face)] = face.split(" ")[1:]
        
    edges = []
    for face in faces:
        for point in face:
            edge = []
            pointSplit = point.split("/")
            secondPoint = face[face.index(point)-1]
            secondPointSplit = secondPoint.split("/")
            edge.append(pointSplit[0])
            edge.append(secondPointSplit[0])
            edges.append(edge)
            
    vertexes = [[float(coord) for coord in vertex] for vertex in vertexes]
    edges = [[vertexes[int(edge[0])-1],vertexes[int(edge[1])-1]] for edge in edges]
    
    return edges
        
    
    
