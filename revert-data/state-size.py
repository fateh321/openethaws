import numpy as np
import matplotlib.pyplot as plt

def addlabels(x,y):
    for i in range(len(x)):
        plt.text(x[i],y[i]+500,y[i], ha = 'center',fontsize = 15) 
# set width of bar
barWidth = 0.25
fig = plt.subplots(figsize =(12, 8))
 
# set height of bar in order: value tran, ERC-20, Swap
Shard1 = [16000, 16044, 48070]
Shard2 = [8000, 8044, 24059]
Shard4 = [4000, 4048, 12054]
 
# Set position of bar on X axis
br1 = np.arange(len(Shard1))
br2 = [x + barWidth for x in br1]
br3 = [x + barWidth for x in br2]

f = plt.figure()
f.set_figwidth(16)
f.set_figheight(7)

addlabels(br1, Shard1)
addlabels(br2, Shard2)
addlabels(br3, Shard4)



# Make the plot
plt.bar(br1, Shard1, color ='teal', width = barWidth,
        edgecolor ='grey', label ='1 Shard')
plt.bar(br2, Shard2, color ='cyan', width = barWidth,
        edgecolor ='grey', label ='2 Shards')
plt.bar(br3, Shard4, color ='lightblue', width = barWidth,
        edgecolor ='grey', label ='4 Shards')
 
# Adding Xticks
plt.xlabel('Transaction type', fontweight ='bold', fontsize = 20)
plt.ylabel('State size', fontweight ='bold', fontsize = 20)
plt.xticks([r + barWidth for r in range(len(Shard1))],
        ['Payment', 'ERC-20', 'Swap'])
 
plt.legend(fontsize = 17)
plt.xticks(fontsize=15)
plt.yticks(fontsize=15)
plt.savefig('state-size.pdf',bbox_inches='tight')
# plt.show()