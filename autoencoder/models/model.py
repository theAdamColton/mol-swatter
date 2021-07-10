# TODO fix imports
import sys

import keras

sys.path.append(".")
import constants


class Model:
    """Generic class for other models to inherit from"""
    def __init__(self):
        self.model = None

    def construct_model(self):
        print("Class should implement this method!")

    def save_model(self, save_path=constants.MODEL_DIR):
        self.autoencoder.save(save_path)

    def load_model(self, load_path=constants.MODEL_DIR):
        print("*******loading model*******")
        try:
            self.autoencoder = keras.models.load_model(load_path)
        except Exception as e:
            print(e)
            return "Error"

    """ Load the training and testing data into this class,
        data should be normalized between 0 and 1
        and should be a vector of dim input_dim"""
    def load(self, x_train, x_test=None):
        self.x_train = x_train
        self.x_test = x_test

    def test_model(self):
        """Models may have to provide their own definitions"""
        input_layer = self.autoencoder.layers[0]
        middle_layer = self.autoencoder.layers[1]
        output_layer = self.autoencoder.layers[-1]
        encoder = keras.Model(input_layer, middle_layer)
        decoder = keras.Model(middle_layer, output_layer)


    def summary(self):
        print("*********SUMMARY************")
        print("*******AUTOENCODER********")
        self.autoencoder.summary()
