# TODO fix imports
import sys
sys.path.append(".")
import constants 

class Model:
    """Generic class for other models to inherit from"""
    def save_model(self, save_path=constants.MODEL_DIR):
        self.model.save(save_path)


    """ Load the training and testing data into this class,
        data should be normalized between 0 and 1
        and should be a vector of dim input_dim"""
    def load(self, x_train, x_test=None):
        assert x_train.shape[1] == self.input_dim
        self.x_train = x_train
        self.x_test = x_test


    def summary(self):
        print("*********SUMMARY************")
        print("*******AUTOENCODER********")
        self.autoencoder.summary()
        print("\n*******ENCODER************")
        self.encoder.summary()
        print("\n*******DECODER************")
        self.decoder.summary()


