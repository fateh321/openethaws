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
t3=0
delta_time=0
for line in f.readlines():      
       
    actual_line = line.rstrip('\n')
    s = re.findall(r"[-+]?\d*\.\d+|\d+", actual_line)
    current_time = ((float(s[0])+float(s[1])/1000000000)-start_time)
    if float(s[2])==287:
        delta_time = current_time
        b_n = float(s[2])
        eff_b_n = float(s[3])
        confirmed_b_n = float(s[4])
        junk_b_n = float(s[5]) 
    if float(s[2]) == 378:
        t1 = current_time-delta_time

    if float(s[2])==383:
        t2 = current_time-delta_time

    if float(s[2])==415:
        t3 = current_time-delta_time    
        
    if float(s[2])>=287 and float(s[2])<450:     
        time.append((float(s[0])+float(s[1])/1000000000)-start_time-delta_time)
        block_no.append(float(s[2])-b_n)
        eff_block_no.append(float(s[3])-eff_b_n)
        if float(s[2])>=303:
            confirmed_block_no.append(float(s[4])-1-confirmed_b_n-16)
        else:
            confirmed_block_no.append(float(s[4])-1-confirmed_b_n)    
        junk_block_no.append(float(s[5])-junk_b_n)
    i+=1 

f = plt.figure()
f.set_figwidth(8)
f.set_figheight(4)

plt.axvline(x=t1, color='r', ls='--',label='malicious block detected')
plt.axvline(x=t2, color='y', ls='--',label='round concluded')
plt.axvline(x=t3, color='g', ls='--',label='susequent round confirmed')
plt.plot(time, confirmed_block_no, label='confirmed blocks')
plt.plot(time, block_no, label='block number')
plt.plot(time, eff_block_no, label='state index')
plt.xlabel("Timestamp(s)")
plt.ylabel("Block number")
plt.legend()


plt.savefig('revert.pdf',bbox_inches='tight')