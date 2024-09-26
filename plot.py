#!/Users/lukap/mambaforge/bin/python

import matplotlib.pyplot as plt
import numpy as np
import sys

args = sys.argv

if len(args) < 2:
    print(f"insufisient number of input arguments {args}")
    quit()
else:
    filename = args[1]


time = []
amplitude = []
lines = []

with open(filename, 'r') as f:
    lines = f.read().splitlines()

for line in lines:
    splitline = line.split(',')
    time.append(float(splitline[0]))
    amplitude.append(float(splitline[1]))

plt.figure()
plt.plot(time, amplitude)
plt.show()
