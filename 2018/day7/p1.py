import sys

class Node:
    def __init__ (self, val):
        self.parents = []
        self.children = []
        self.value = val
        self.lookedAt = False

    def __repr__(self):
        return ("%s %s" % (self.value, self.children))

    def isValid(self):
        self.parents.sort(key=lambda x: x.value)
        for node in self.parents:
            if not node.lookedAt:
                return False
        return True

    def getCause(self):
        self.parents.sort(key=lambda x: x.value)
        for node in self.parents:
            if not node.lookedAt:
                return node


lines = sys.stdin.readlines()
#lines = open('/home/alex/Coding/AdventOfCode2018/day7/input.txt', 'r').readlines()

# parse the input
nodes = {}
for line in lines:
    gt = line[5]
    lt = line[36]
    #print(gt, lt)
    if not gt in nodes:
        nodes[gt] = Node(gt)
    if not lt in nodes:
        nodes[lt] = Node(lt)
    nodes[gt].children.append(lt)
    nodes[lt].parents.append(gt)

# find the root
listofRoots = []
for key, node in nodes.items():
    if len(node.parents) == 0:
        listofRoots.append(key)
        #print(node.value)

pq = sorted(listofRoots)
while(len(pq) > 0):
    node = pq.pop(0)
    print(node, end='')
    for key, value in nodes.items():
        if node in value.parents:
            value.parents.remove(node)
            if len(value.parents) == 0:
                pq.append(key)
    pq.sort()

print('')