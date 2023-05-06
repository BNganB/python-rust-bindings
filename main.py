from lib import numpyrust
import numpy as np
import random
import time

TEST_ARRAY_LEN = 100

def generate_array(TEST_ARRAY_LEN):
    global test_array
    test_array = []
    for i in range(TEST_ARRAY_LEN):
        test_array.append(random.randint(0,9))



def timer(func):
    def wrapper(*args, **kwargs):
        start_time = time.time()
        result = func(*args, **kwargs)
        end_time = time.time()
        print(f"{func.__name__} took {(end_time - start_time)*1000} ms to run\nResult = {result}")
        return result
    return wrapper


@timer
def rust_array(TEST_ARRAY):
    return numpyrust.array(TEST_ARRAY)

@timer
def c_numpy_array(TEST_ARRAY):
    return np.array(TEST_ARRAY).tolist()

def main():
    generate_array(TEST_ARRAY_LEN)
    rust_array(test_array)
    c_numpy_array(test_array)

main()
