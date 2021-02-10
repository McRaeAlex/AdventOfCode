import sys

lines = sys.stdin.readlines()

# Parse the input
coords = []
infinite = {}
areas = {(-1,-1):0}

for line in lines:
    data = line.split(',')
    coord = (int(data[0]), int(data[1]))
    coords.append(coord)
    infinite[coord] = False
    areas[coord] = 0


for x in range(500):
    for y in range(500):
        minCoord = (0,0)
        minDist = 1001
        for coord in coords:
            dist = abs(coord[0] - x) + abs(coord[1] - y)
            if dist < minDist:
                minDist = dist
                minCoord = coord
            elif dist == minDist:
                minCoord = (-1,-1)
        if x == 499 or x == 0 or y == 499 or y == 0:
            infinite[minCoord] = True
        areas[minCoord] += 1

maxA = 0
for key, value in areas.items():
    if value > maxA and not infinite[key]:
        maxA = value

print(maxA)