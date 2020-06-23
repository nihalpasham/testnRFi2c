# testnRFi2c
Testing i2c communciation between the `nRF52840-mdk board` (running the Rust nRF52840_hal) and the `AT88CKSCKTSOIC-XPRO (Microchip XPRO) extension board`.
 - **Picture below contains the following:**
   - The AT88CKSCKTSOIC-XPRO (Microchip XPRO) extension board (the one in red to the left) 
   - The extension board is just a passthrough that `connects the actual i2c slave via an SOIC port` (i.e. the black thing on the read board). 
   - The nRF52840-mdk board running our example code (testnRFi2c).
   - Screenshot of test results
     - Unable to transfer 2 zero bytes. The TWIM peripheral's AMOUNT register reports no bytes were sent. 

![Debuuging Output](https://github.com/nihalpasham/testnRFi2c/blob/master/2020-06-23.png)
