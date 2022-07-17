import queue
from queue import Queue
import time
import threading
from threading import Thread
import random

q = Queue()

def long_task(num):
    time.sleep(.1)
    if random.randint(0, 300) % 100 == 0:
        raise ValueError("this should be retried")
    return num

for i in range(1000):
    task = {"num": i}
    q.put(task)

print(f"Q size: {q.qsize()}")

def worker(task_queue: Queue):
    name = threading.current_thread().name
    while not task_queue.empty():
        task = task_queue.get()
        print(f"Worker {name} got task {task}")
        try:
            result = long_task(task["num"])
        except ValueError:
            print("retrying")
            task_queue.put(task)
        else:
            print(f"Worker {name} task done: {result}")
        finally:
            task_queue.task_done()


    print(f"Worker {name} finished. Exitting")

threads = [Thread(target=worker, args=(q, ), name=f"Thread {i}") for i in range(32)]
[t.start() for t in threads]
q.join()