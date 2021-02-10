
class Node:
    def __init__(self, value, clockwise, counter):
        self.value = value
        self.clockwise = clockwise
        self.counter = counter


lastMarble = 71730 * 100
numberOfPlayers = 464

scores = {}
for i in range(numberOfPlayers):
    scores[i] = 0

current = Node(0, None, None)
current.clockwise = current
current.counter = current

for i in range(1, lastMarble):
    if i % 23 == 0:
        # add the current marble
        scores[i % numberOfPlayers] += i
        # go 7 back
        for _ in range(7):
            current = current.counter
        # add the value
        scores[i % numberOfPlayers] += current.value
        # remove it
        current.clockwise.counter = current.counter
        current.counter.clockwise = current.clockwise
        current = current.clockwise
    else:
        new = Node(i, current.clockwise.clockwise, current.clockwise)
        new.clockwise.counter = new
        new.counter.clockwise = new
        current = new

maxVal = 0
for key, value in scores.items():
    if value > maxVal:
        maxVal = value

print(maxVal)
