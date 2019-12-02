# fuel = floor( mass / 3 ) - 2

def calcFuel(fuel):
    fuel_n = fuel // 3 - 2
    if fuel_n > 0:
        return fuel_n + calcFuel(fuel_n)
    else:
        return 0

file = open("input.txt", "r")

fuel_sum = 0

for line in file.readlines():
    fuel_sum += calcFuel(int(line))


print(fuel_sum)

file.close()

