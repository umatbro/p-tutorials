from functools import wraps


def memoize(func):
    func.cache = {}
    def _wrapper(*args):
        if not args in func.cache:
            func.cache[args] = func(*args)

        return func.cache[args]

    return _wrapper

@memoize
def fibo(n):
    if n < 2:
        return n
    else:
        return fibo(n - 1) + fibo(n - 2)


for i in range(1200):
    print('Fibo {}: {}'.format(i, fibo(i)))
    
