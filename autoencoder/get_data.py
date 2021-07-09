"""Generates numpy ndarrays for IR spectral data parsed from .jdx files
in a directory"""

import numpy as np
import mol_swatter
import os


class Get_IRData:
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
        self.data = np.ndarray(shape=(self.dimensions,))
        # Calls self.__add_ndarry_to_data on each jdx file in the directory
        __iterate_over_files(self.directory, ".jdx", self.__add_ndarry_to_data)

        nfiles = self.data.ndim
        print("IR spectrum from {} to {} by {}, for {} files".format(self.first_x, self.last_x, self.dimensions, nfiles))

        return self.data

    def __add_jdx_to_data(self, filepath):
        new_array = self.__get_ndarray_from_jdx(filepath)
        self.__add_ndarry_to_data(new_array)

    def __add_ndarry_to_data(self, new_array):
        self.data = np.stack(self.data, new_array)

    def __get_ndarray_from_jdx(self, filepath):
        spectrum = mol_swatter.Spectrum(filepath)
        if not spectrum.is_valid():
            print("Invalid spectrum : " + filepath)
            return
        spectrum = spectrum.transform(self.first_x, self.last_x, self.dimensions)
        y_values = spectrum.get_y_values()
        assert y_values.len() == self.dimensions
        return np.array(y_values)


def __iterate_over_files(directory, extension, funct):
    for filename in os.listdir(directory):
        if filename.endswith(extension):
            file_path = os.path.join(directory, filename)
            funct(file_path)


"""For testing"""
getter = Get_IRData("../scraper/raw_data", 800, 3000, 128)
data = getter.get_ir_data()
