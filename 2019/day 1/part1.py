# fuel = floor( mass / 3 ) - 2

file = open("input.txt", "r")

fuel_sum = 0

for line in file.readlines():
    fuel_sum += int(line) // 3 - 2


print(fuel_sum)

file.close()
