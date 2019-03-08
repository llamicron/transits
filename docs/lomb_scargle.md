# 3.1 Searching a set of light curves for periodic variable stars with Lomb-Scargle

## Example
This one will print (in a human-readable way) the period information
```
$ vartools -i EXAMPLES/1 -LS 0.01 100.0 0.1 1 0 -oneline

Name                     = EXAMPLES/1
LS_Period_1_0            =    77.76775250
Log10_LS_Prob_1_0        = -5709.91013
LS_Periodogram_Value_1_0 =    0.99392
LS_SNR_1_0               =  386.76802
```

This one will output a `.ls` file to `data/out`. You can also replace `-header -numbercolumns` with `-oneline` as above to make the output more human readable.
```
$ vartools -l data/in/lc_list -LS 0.01 100.0 0.1 5 1 data/out whiten -header -numbercolumns

#1_Name 2_LS_Period_1_0 3_Log10_LS_Prob_1_0 4_LS_Periodogram_Value_1_0 5_LS_SNR_1_0 6_LS_Period_2_0 7_Log10_LS_Prob_2_0 8_LS_Periodogram_Value_2_0 9_LS_SNR_2_0 10_LS_Period_3_0 11_Log10_LS_Prob_3_0 12_LS_Periodogram_Value_3_0 13_LS_SNR_3_0 14_LS_Period_4_0 15_Log10_LS_Prob_4_0 16_LS_Periodogram_Value_4_0 17_LS_SNR_4_0 18_LS_Period_5_0 19_Log10_LS_Prob_5_0 20_LS_Periodogram_Value_5_0 21_LS_SNR_5_0
data/in/1.transit     2.08772490  -30.67333    0.04543   37.44174     0.19297209  -21.66587    0.03376   27.49300     0.27002692  -19.28114    0.03058   25.66263     0.30890865  -18.39319    0.02948   25.36384     0.17694597  -13.88719    0.02357   20.75944
```

## Commands
Here are the commands from the reference site
```
-LS
    minp maxp subsample Npeaks
    operiodogram [outdir] ["noGLS"]
    ["whiten"] ["clip" clip clipiter]
    ["fixperiodSNR" <"aov" | "ls" | "injectharm" | "fix" period
        | "list" ["column" col] | "fixcolumn" <colname | colnum>>]
    ["bootstrap" Nbootstrap]
```


## Data
### Input
Light curves should be in this format:
```
# Time[BJD]  Mag      Err
53725.173920 10.44080 0.00136
53725.176540 10.43881 0.00166
53725.177720 10.44024 0.00145
...
```
