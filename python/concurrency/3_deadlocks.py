import time
import threading
from threading import Thread
import random
import sys

ACCOUNTS = {
    "a1": 1000,
    "a2": 1000,
}

ITERATIONS = {
    "a1": 0,
    "a2": 0
}

def move_funds(_from, _to, expected):
    name = threading.current_thread().name

    while True:
        amount = random.randint(-100, 100)
        ACCOUNTS[_from] -= amount
        ACCOUNTS[_to] += amount
        total = sum(ACCOUNTS.values())
        if total != expected:
            print(f"{name} found an inconsistent balance: {total}")
            break
        print(f"Balance: {ACCOUNTS}")
        print(f"Iterations: {ITERATIONS}")
        ITERATIONS[_from] += 1

t1 = Thread(target=move_funds, args=("a1", "a2", 2000))
t2 = Thread(target=move_funds, args=("a2", "a1", 2000))

t1.start()
t2.start()