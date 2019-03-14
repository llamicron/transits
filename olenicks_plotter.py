'''
not working :(

readnplot.py
reads VARTOOLS files and plots graphs
'''

from numpy import loadtxt
from numpy import zeros
from matplotlib.pyplot import *

name = raw_input("Enter the name of the file: ")  #Be sure to add .model to file name

data = loadtxt(name,skiprows = 1)
MJD = data[:,0]
OBS = data[:,1]
MOD = data[:,2]
ERR = data[:,3]
PHZ = data[:,4]

plotname = name[:11]
figure(1)
fig1 = figure(1)
fig1.patch.set_facecolor('white')
plot(MJD, OBS, 'k+', alpha = 0.5)
title(plotname)
xlabel("MJD")
ylabel("Observed Magnitude")
ylim(ymax = 13.0, ymin = 12.5)
ax = gca()
ax.set_ylim(ax.get_ylim()[::-1])   #reverse the y-axis direction
grid(False)

figure(2)
fig = figure(2)
fig.patch.set_facecolor('white')

for i in range(0, len(PHZ)):
    if PHZ[i] <= 0.5:
        PHZ[i] += 1.0
    PHZ[i] -= 1.0

# xlim([-0.1,+0.1])

errorbar(PHZ,OBS, yerr=0.017, color='k',linewidth = 0.3, alpha = 0.5, zorder = 1)
# plot(PHZ,OBS,"k+", alpha=0.2)
plot(PHZ,MOD,"rx", alpha = 0.5, zorder = 10)



title(plotname)
xlabel("Phase")
ylabel("Model")
ylim(ymax = 13.0, ymin = 12.5)
ax = gca()
ax.set_ylim(ax.get_ylim()[::-1])   #reverse the y-axis direction
grid(False)
show()
