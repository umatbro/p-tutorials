import multiprocessing as mp
from multiprocessing import Process
import random
import time
import threading
import queue

work_to_do = mp.JoinableQueue()
work_done = mp.SimpleQueue()

numbers = [
    15492781,
    15492787,
    15492803,
    15492811,
    15492810,
]

def is_prime(n):
    if n in {2, 3}:
        return True
    if n % 2 == 0:
        return False
    for divisor in range(3, n, 2):
        if n % divisor == 0:
            return False
    return True


[work_to_do.put(num) for num in numbers]

MAX_WORKERS = 5

def process_consumer(task_queue, results_queue):
    while True:
        try:
            number = task_queue.get_nowait()
            result = is_prime(number)
            results_queue.put([number, result])
        except queue.Empty:
            print("No more numbers. Exitting")
            return

process_pool = [Process(target=process_consumer, args=(work_to_do, work_done)) for _ in range(MAX_WORKERS)]

[p.start() for p in process_pool]
[p.join() for p in process_pool]
[p.close() for p in process_pool]

res = []
while not work_done.empty():
    res.append(work_done.get())
print(res)

##########################
# PIPES
##########################
main_conn, worker_conn = mp.Pipe()
def process_prime_worker(number, pipe_connection):
    result = is_prime(number)
    pipe_connection.send([number, result])

processes = [
    Process(
        target=process_prime_worker,
        args=(number, worker_conn)
    ) for number in numbers
]


############################
# Process pools
############################
with mp.Pool(processes=4) as pool:
    n = random.choice(numbers)
    result = pool.apply_async(is_prime, (n, ))
    print(f"Number {n} is prime? {result.get()}")