"""Generates numpy ndarrays for IR spectral data parsed from .jdx files
in a directory"""
import sys
import constants
sys.path.append(constants.BIN_DIR)
import numpy as np
import mol_swatter
import os


def get(directory, first_x=800, last_x=3000, dimensions=256, training_data=1.5):
    """
    Get a ndarray from the spectral data from
    all .jdx files in the given dir.

    first_x and last_x specify the range of the wavelength in cm-1

    dimensions specifies the number of data points

    x_test can be specified as a ratio of files to be
        picked as training data.
    Returns a tuple:
        (x_train, x_test)
    """
    data = None
    # Calls .__add_ndarry_to_data on each jdx file in the directory
    for filename in os.listdir(directory):
        if filename.endswith(".jdx"):
            filename = os.path.join(directory, filename)
            res = __add_jdx_to_data(filename, first_x, last_x, dimensions, data)
            if res is not None:
                data = res

    nfiles = data.shape[0]
    print(
        "IR spectrum data from {} to {} by {}, for {} files".format(
            first_x, last_x, dimensions, nfiles
        )
    )
    # The tuple is for when the testing data is implemented
    return (data,)


def __add_jdx_to_data(filepath, first_x, last_x, dimensions, data):
    new_array = __get_ndarray_from_jdx(filepath, first_x, last_x, dimensions)
    # If the file was invalid
    if new_array is None:
        return
    return __add_ndarry_to_data(new_array, data)


def __add_ndarry_to_data(new_array, data):
    # Initializes .data for the first array to be added
    if data is None:
        data = np.array([new_array])
    elif data.ndim == 1:
        assert data.size == new_array.size
    elif data.ndim > 1:
        assert data[-1].size == new_array.size, "{} is not {}".format(
            data[-1].size, new_array.size
        )
    # Appends the new array
    data = np.append(data, [new_array], axis=0)
    return data


def __get_ndarray_from_jdx(filepath, first_x, last_x, dimensions):
    spectrum = mol_swatter.Spectrum(filepath)
    if not spectrum.is_valid():
        print("Invalid spectrum : " + filepath)
        return
    spectrum = spectrum.transform(first_x, last_x, dimensions)
    y_values = spectrum.get_y_values()
    assert len(y_values) == dimensions
    return np.array(y_values)


if __name__ == "__main__":
    """For testing"""
    data = get("../scraper/raw_data", 800, 3000, 128)
