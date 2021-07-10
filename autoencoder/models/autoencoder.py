import keras
from keras import layers
import numpy as np

""" Simple autoencoder """


class Autoencoder:
    def __init__(self, input_dim, latent_dim):
        self.input_dim = input_dim
        self.latent_dim = latent_dim
        # Encoder uses a singe dense layer to encode and decode
        input_layer = keras.Input(shape=input_dim)
        encoded = layers.Dense(latent_dim, activation="relu")(input_layer)
        decoded = layers.Dense(input_dim, activation="sigmoid")(encoded)
        self.autoencoder = keras.Model(input_layer, decoded)

        self.encoder = keras.Model(input_layer, encoded)

        # Defines the decoder
        encoded_input = keras.Input(shape=latent_dim)
        decoder_layer = self.autoencoder.layers[-1]
        self.decoder = keras.Model(encoded_input, decoder_layer(encoded_input))

        self.autoencoder.compile(optimizer="adadelta", loss="binary_crossentropy")

    """ Load the training and testing data into this class,
        data should be normalized between 0 and 1
        and should be a vector of dim input_dim"""

    def load(self, x_train, x_test=None):
        assert x_train.shape[1] == self.input_dim
        self.x_train = x_train
        self.x_test = x_test

    def train(self, batch_size=128):
        self.autoencoder.fit(
            self.x_train, self.x_train, batch_size=batch_size, shuffle=True
        )

    def summary(self):
        print("*********SUMMARY************")
        print("*******AUTOENCODER********")
        self.autoencoder.summary()
        print("\n*******ENCODER************")
        self.encoder.summary()
        print("\n*******DECODER************")
        self.decoder.summary()


"Tests this class"
if __name__ == "__main__":
    autoencoder = Autoencoder((128), 20)
    autoencoder.summary()
