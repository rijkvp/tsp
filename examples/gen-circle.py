#!/bin/python
import math, sys, random
count = int(sys.argv[1])
cities = []
for i in range(0, 360, 360//count):
    cities.append((math.sin(math.radians(i)) * 250, math.cos(math.radians(i)) * 250))
random.shuffle(cities)
for (x, y) in cities:
    print(x, y)
