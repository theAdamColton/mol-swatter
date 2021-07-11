"""This test should be run from the base directory"""

import sys

sys.path.append(".")
from benchmark import benchmark
from get_ir_data import get
from constants import DATA_DIR


def test_get_ir_data():
    print("Benchmarking get_ir_data()")
    res = benchmark(get, args=(DATA_DIR, 800, 3000, 256, 0))


if __name__ == "__main__":
    test_get_ir_data()
