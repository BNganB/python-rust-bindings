from lib import numpyrust
import numpy as np
import random
import time

TEST_ARRAY_LEN = 10000
TEST_START = 0
TEST_STOP = 1000000000000000000
TEST_ARRAY = []

start = 0
stop = 20
step = 10
vec1 = [1, 2, 3, 4]
vec2 = [1, 2, 0, 4]


def generate_array(TEST_ARRAY_LEN):
    global TEST_ARRAY
    for i in range(TEST_ARRAY_LEN):
        TEST_ARRAY.append(random.randint(0, 9))


def timer(func):
    def wrapper(*args, **kwargs):
        start_time = time.time()
        result = func(*args, **kwargs)
        end_time = time.time()
        print(f"{func.__name__} took {(end_time - start_time)*1000} ms to run")
        print(f"\n{func.__name__} Result = {result}\n")
        return result
    return wrapper


@timer
def rust_array(TEST_ARRAY):
    return numpyrust.array(TEST_ARRAY)
    # consistently faster than c past 5 fig. len generation


@timer
def c_numpy_array(TEST_ARRAY):
    return np.array(TEST_ARRAY).tolist()


@timer
def rust_randint(start, stop):
    return numpyrust.randint(start, stop)
    # WHY ARE YOU SLOWER


@timer
def c_randint(start, stop):
    return random.randint(start, stop)


@timer
def c_linspace(start, stop, step):
    return np.linspace(start, stop, step)


@timer
def rust_linspace(start, stop, step):
    return numpyrust.linspace(start, stop, step)


@timer
def rust_equal(vec1, vec2):
    return numpyrust.equal(vec1, vec2)


@timer
def c_equal(vec1, vec2):
    return np.equal(vec1, vec2)




if __name__ == "__main__":
    rust_equal(vec1, vec2)
    c_equal(vec1, vec2)
