"""Generates numpy ndarrays for IR spectral data parsed from .jdx files
in a directory"""

import numpy as np
import mol_swatter
import os


class Get_IRData:
    """
    Getter for converting the spectral data in .jdx files into a huge ndarray
    """

    def __init__(self, directory, first_x, last_x, dimensions):
        self.directory = directory
        self.first_x = first_x
        self.last_x = last_x
        self.dimensions = dimensions

    def get_ir_data(self, training_data=0.0):
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
        assert training_data == 0, "training ratio is not implemented"
        self.data = np.ndarray(shape=(0,))
        # Calls self.__add_ndarry_to_data on each jdx file in the directory
        iterate_over_files(self.directory, ".jdx", self.__add_jdx_to_data)

        nfiles = self.data.ndim
        print(
            "IR spectrum from {} to {} by {}, for {} files".format(
                self.first_x, self.last_x, self.dimensions, nfiles
            )
        )

        return self.data

    def __add_jdx_to_data(self, filepath):
        new_array = self.__get_ndarray_from_jdx(filepath)
        # If the file was invalid
        if new_array is None:
            return
        self.__add_ndarry_to_data(new_array)

    def __add_ndarry_to_data(self, new_array):
        # Initializes self.data for the first array to be added
        if self.data.size == 0:
            self.data = np.array([new_array])
            return
        elif self.data.ndim == 1:
            assert self.data.size == new_array.size
        elif self.data.ndim > 1:
            assert self.data[-1].size == new_array.size, "{} is not {}".format(
                self.data[-1].size, new_array.size
            )
        # Appends the new array
        self.data = np.append(self.data, [new_array], axis=0)

    def __get_ndarray_from_jdx(self, filepath):
        spectrum = mol_swatter.Spectrum(filepath)
        if not spectrum.is_valid():
            print("Invalid spectrum : " + filepath)
            return
        spectrum = spectrum.transform(self.first_x, self.last_x, self.dimensions)
        y_values = spectrum.get_y_values()
        assert len(y_values) == self.dimensions
        return np.array(y_values)


def iterate_over_files(directory, extension, funct):
    for filename in os.listdir(directory):
        if filename.endswith(extension):
            file_path = os.path.join(directory, filename)
            funct(file_path)


if __name__ == "__main__":
    """For testing"""
    getter = Get_IRData("../scraper/raw_data", 800, 3000, 128)
    data = getter.get_ir_data()
