import matplotlib.pyplot as plt
import numpy as np


time = []
amplitude = []
lines = []

with open('audiofile.txt', 'r') as f:
    lines = f.read().splitlines()
    
for line in lines:
    splitline = line.split(',')
    time.append(float(splitline[0]))
    amplitude.append(float(splitline[1]))

plt.figure()
plt.plot(time, amplitude)
plt.show()
