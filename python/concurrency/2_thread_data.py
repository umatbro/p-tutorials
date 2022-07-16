import threading
import time
from threading import Thread
from dataclasses import dataclass, field

@dataclass
class PushupStatus:
    count: int
    lock: threading.Lock = field(default_factory=threading.Lock)

    def __repr__(self):
        return str(self.count)

COUNTER = {
    "mat": PushupStatus(0),
    "domi": PushupStatus(0),
}


def pushup(who):
    current = COUNTER[who]
    print(f"Current {who} pushups are {current.count}")
    if current.lock.locked():
        print(f"{who} is locked, waiting...")
    
    with current.lock:
        time.sleep(1)
        new_val = current.count + 1
        COUNTER[who].count = new_val
        print(f"{who} pushups set to {new_val}")

m1 = Thread(target=pushup, args=("mat", ))
d1 = Thread(target=pushup, args=("domi", ))
m1.start(), d1.start()
time.sleep(0.5)
m2 = Thread(target=pushup, args=("mat",))
m2.start()

m1.join(), m2.join(), d1.join()

print(COUNTER)
