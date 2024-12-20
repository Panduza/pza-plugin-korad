import logging
from panduza import Reactor

"""
This test suppose that only 1 KD3005P is on the bench
"""

# Start logging
logging.basicConfig(level=logging.DEBUG)

# Create Panduza reactor
print("start connection")
r = Reactor()
r.start()
print("connection ok")

voltage_control = r.attribute_from_name("voltage")

print("min ----", )
print("max ----", )

# 
step = 1
if voltage_control.decimals() != 0:
    step = 1 / (10 ** voltage_control.decimals())


i = voltage_control.min()
while i <= voltage_control.max():
    print(f"set voltage to {i}{voltage_control.unit()}")
    i += step
    voltage_control.set(i)



