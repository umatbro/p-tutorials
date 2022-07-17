# processes are more expensive than threads
import multiprocessing as mp
from multiprocessing import Process
import random
import time
import threading
import queue

def say_hello():
    myself = mp.current_process()
    print(f"Hello from {myself.name}")

p = Process(target=say_hello, name="My first process")
p.start()
p.join()
p.close()

def is_prime(n):
    if n in {2, 3}:
        return True
    if n % 2 == 0:
        return False
    for divisor in range(3, n, 2):
        if n % divisor == 0:
            return False
    return True

numbers = [
    15492781,
    15492787,
    15492803,
    15492811,
    15492810,
]

def check_prime_worker(number):
    if is_prime(number):
        print(f"Number {number} IS PRIME ✅")
    else:
        print(f"Number {number} is NOT prime :(")


processes = [Process(target=check_prime_worker, args=(num,)) for num in numbers]

start = time.time()
[p.start() for p in processes]
[p.join() for p in processes]
end = time.time()
[p.close() for p in processes]
print(f"Multiprocessing time: {end - start}")

start = time.time()
for num in numbers:
    check_prime_worker(num)
end = time.time()
print(f"Sequential time: {end - start}")