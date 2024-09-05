import numpy as np

def spreadPoints(spacing:float, start:tuple, end:tuple) -> list:
    # Define the start and end points of the line
    startPoint = np.array(start)
    endPoint = np.array(end)

    # Compute the direction vector of the line
    direction = endPoint - startPoint

    # Define the distance between the start and end points
    lineDistance = np.linalg.norm(direction)

    # Define the number of segments you want to divide the line into
    numSegments = int(lineDistance / spacing)

    # Compute the spacing vector
    spacingVector = direction / numSegments

    # Initialize the list of points
    points = [startPoint]

    # Compute the remaining distance to the end point
    remainingDistance = lineDistance - spacing

    # Evenly distribute points along the line
    for i in range(numSegments):
        # Move to the next point
        points.append(points[-1] + spacingVector)
        
        # Update the remaining distance
        remainingDistance -= spacing
        
        # If we've reached the end point, stop
        if remainingDistance < spacing:
            points.append(endPoint)
            break
    points = [point.tolist() for point in points]
    return points