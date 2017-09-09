import time

def my_timer(original_function):
    def wrapper(*args, **kwargs):
        start = time.time()
        result = original_function(*args, **kwargs)
        total = time.time() - start
        print('{} ran in {} sec'.format(original_function.__name__, total))

        return result

    return wrapper


@my_timer
def display_info(name, age):
    time.sleep(1)
    print('{} is {} years old'.format(name, age))


display_info('Mat', 3)
