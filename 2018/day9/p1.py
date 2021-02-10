lastMarble = 71730
numberOfPlayers = 464

scores = {}
for i in range(numberOfPlayers):
    scores[i] = 0

marbles = [0]

index = 0
for marble in range(1, lastMarble + 1):
    if marble % 23 == 0:
        scores[marble % numberOfPlayers] += marble
        index -= 7
        index = index % len(marbles)
        scores[marble % numberOfPlayers] += marbles.pop(index)
    else:
        index = (index + 2) % len(marbles)
        marbles.insert(index, marble)

maxVal = 0
for key, value in scores.items():
    if value > maxVal:
        maxVal = value

print(maxVal)
