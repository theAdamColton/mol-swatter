import time


def benchmark(function, args=None):
    """Pass a function to time it"""
    print("*******            *******")
    start = time.time() * 1000
    function(*args)
    end = time.time() * 1000
    taken = end-start
    print("*******{:^10.4f}ms*******".format(taken))


