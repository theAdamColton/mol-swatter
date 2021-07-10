import keras
from keras import layers
import numpy as np
# TODO pull my hair out over imports
import sys
sys.path.append("./models/")
from model import Model


class Autoencoder(Model):
    """Simple autoencoder"""
    def construct_model(self, input_dim, latent_dim):
        """Constructs self.model"""
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

    def train(self, batch_size=128, epochs=-1):
        self.autoencoder.fit(
            self.x_train,
            self.x_train,
            batch_size=batch_size,
            shuffle=True,
            epochs=epochs,
            validation_data=(self.x_test, self.x_test)
        )

    
"Tests this class"
if __name__ == "__main__":
    autoencoder = Autoencoder((128), 20)
    autoencoder.summary()
