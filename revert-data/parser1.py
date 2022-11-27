import re
import matplotlib.pyplot as plt

time = []
block_no = []
eff_block_no = []
confirmed_block_no = []
junk_block_no = []
f = open("data-3.csv", "r")

fl = f.readline()
fl_n = re.findall(r"[-+]?\d*\.\d+|\d+", fl)
start_time = float(fl_n[0])+float(fl_n[1])/1000000000
i = 0
t1=0
t2=0
for line in f.readlines():      
       
    actual_line = line.rstrip('\n')
    s = re.findall(r"[-+]?\d*\.\d+|\d+", actual_line)
    current_time = ((float(s[0])+float(s[1])/1000000000)-start_time)
    if float(s[2])==175:
        t1 = current_time

    if float(s[3])==175:
        t2 = current_time
        
    if current_time<750:     
        time.append((float(s[0])+float(s[1])/1000000000)-start_time)
        block_no.append(float(s[2]))
        eff_block_no.append(float(s[3]))
        confirmed_block_no.append(float(s[4])-1)
        junk_block_no.append(float(s[5]))
    i+=1 

plt.axvline(x=t1, color='r', ls='--',label='axvline - full height')
plt.axvline(x=t2, color='r', ls='--',label='axvline - full height')

plt.plot(time, confirmed_block_no)
plt.plot(time, block_no)
plt.plot(time, eff_block_no)
plt.show()