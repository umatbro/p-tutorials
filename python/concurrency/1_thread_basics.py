import time
import random
import threading
from threading import Thread


def simple_worker():
    to_sleep = random.random() * 5
    myself = threading.current_thread()
    ident = threading.get_ident()
    print(f"Ill sleep for {to_sleep}. I'm thread {myself}, ident {ident}", flush=True)
    time.sleep(to_sleep)
    value = random.randint(0, 99)
    print(f"My value {value}", flush=True)

# t1 = Thread(target=simple_worker)

# threads = [Thread(target=simple_worker, name=str(i)) for i in range(5)]

# [t.start() for t in threads]
# [t.join() for t in threads]

# for t in threads:
#     print(f"{t.name}: {t.ident}")


#######################################
# def simple_args(time_to_sleep):
#     myself = threading.current_thread()
#     ident = threading.get_ident()
#     print(f"I am a thread {myself.name} and I am sleeping for {time_to_sleep}")
#     time.sleep(time_to_sleep)
#     print(f"Thread {myself.name} exiting...")

# t1 = Thread(target=simple_args, name="Bubbles", args=(3, ))
# t2 = Thread(target=simple_args, name="Blossom", args=(1.5, ))
# t3 = Thread(target=simple_args, name="Buttercup", args=(2, ))

# t1.start(), t2.start(), t3.start()

class MyThread(Thread):
    def __init__(self, time_to_sleep, name=None):
        super().__init__(name=name)
        self.time_to_sleep = time_to_sleep
    
    def run(self):
        ident = threading.get_ident()
        print(f"I am thread {self.name} (ID: {ident}), and I'm sleeping for {self.time_to_sleep} secs")
        time.sleep(self.time_to_sleep)
        print(f"Thread {self.name} exitting...")

t = MyThread(2)
t.start()