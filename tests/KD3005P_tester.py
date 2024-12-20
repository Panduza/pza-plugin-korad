import time
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


pp = r.attribute_from_name("voltage")





print("min ----", pp.min())
print("max ----", pp.max())

pp.set(4)




time.sleep(2)

