# TODO fix imports
import sys

import tensorflow.keras as keras

sys.path.append(".")
import constants
from plot import plot

import json


class Model:
    """Generic class for other models to inherit from"""

    def __init__(self):
        self.model = None

    def construct_model(self):
        print("Class should implement this method!")

    def save_model(self, save_path=constants.MODEL_DIR):
        self.autoencoder.save(save_path)
        self.__save_conf(save_path + "/conf.json")

    def load_model(self, load_path=constants.MODEL_DIR):
        self.autoencoder = keras.models.load_model(load_path)
        print("*******loaded model*******")
        assert self.__load_conf(load_path + "/conf.json")==None

    def __save_conf(self, save_file):
        tofile = json.dumps(self.__m_conf)
        file = open(save_file, "w")
        file.write(tofile)
        file.close()
        print("*******saved configuration**********")

    def __load_conf(self, load_file):
        try:
            file = open(load_file)
            tempconf = file.read()
            self.__m_conf = json.loads(tempconf)
        except Exception as e:
            print(e)
            return "Error"
        print("*******loaded configuration*********")

    def load(self, x_train, x_test=None, startx=None, lastx=None):
        """Load the training and testing data into this class,
        data should be normalized between 0 and 1
        and should be a vector of dim input_dim"""
        self.x_train = x_train
        self.x_test = x_test
        self.__m_conf = {
                'startx': startx,
                'lastx' : lastx,
                }

    def test_model(self):
        assert self.__m_conf != {}
        encoder, decoder = self.__run_test()
        encoded_vec = encoder.predict(self.x_test)
        decoded_vec = decoder.predict(encoded_vec)
        self.__show_test_result(decoded_vec)

    def __show_test_result(self, decoded_vectors):
        """Models should not privide their own definition"""
        xmin = self.__m_conf.get('startx')
        xmax = self.__m_conf.get('lastx')
        npoints = self.autoencoder[0].input.shape[0]
        n = 10
        for i in range(n):
            print("plotting test vector")
            plot(xmin, xmax, npoints, self.x_test[i])
            print("plotting reconstructed vector")
            plot(xmin, xmax, npoints, decoded_vectors[i])

    def __run_test(self):
        """Models may have to provide their own definitions"""
        input_layer = self.autoencoder.layers[0]
        middle_layer = self.autoencoder.layers[1]
        output_layer = self.autoencoder.layers[-1]
        encoder = keras.Model(input_layer, middle_layer)
        decoder = keras.Model(middle_layer, output_layer)
        return (encoder, decoder)

    def summary(self):
        print("*********SUMMARY************")
        print("*******AUTOENCODER********")
        self.autoencoder.summary()
