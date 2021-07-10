import matplotlib.pyplot as plt
import numpy as np


def plot(x_min, x_max, npoints, y_values):
    """Plots a single spectrum"""
    x_values = __gen_x_vals(x_min, x_max, npoints)
    plt.plot(x_values, y_values)
    ax = plt.gca()
    ax.set_xlim(ax.get_xlim()[::-1])
    plt.show()


def __gen_x_vals(x_min, x_max, npoints):
    delta_x = (x_max - x_min) / npoints
    x_values = np.array(x_min)
    curr_x = x_min
    while x_values.size < npoints:
        curr_x += delta_x
        np.append(x_values, curr_x)
    return x_values
