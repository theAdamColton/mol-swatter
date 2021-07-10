"""
Runs the autoencoder
"""

import sys
from models.autoencoder import Autoencoder
import argparse
import constants
import get_ir_data

MODELS = {"simple": Autoencoder}


class Run:
    def __init__(self, args):
        parser = self.__get_arg_parser()
        args = parser.parse_args(args)

        print("\n---Running---\n{}".format(args))
        self.args = args
        self.model = MODELS.get(args.model)()
        self.__run_encoder()

    def __run_encoder(self):
        data = get_ir_data.get(
            self.args.data_dir,
            self.args.firstx,
            self.args.lastx,
            self.args.input_dim,
        )
        if self.args.load:
            res = self.model.load_model()
            if res is not None:
                self.__construct_model()
        else:
            self.__construct_model()
        xtrain = data[0]
        print(xtrain)
        self.model.load(xtrain)
        self.model.summary()
        while True:
            self.model.train(self.args.batchsize, self.args.epochs)
            self.model.save_model()
            print("Finished Epoch")

    def __construct_model(self):
        self.model.construct_model(self.args.input_dim, self.args.latent_dim)

    def __get_arg_parser(self):
        parser = argparse.ArgumentParser(description="Run an autoencoder")
        parser.add_argument("--model", "-m", default="simple", choices=MODELS)
        parser.add_argument("--batchsize", "-b", default=512, type=int)
        parser.add_argument("--data_dir", "-d", default=constants.DATA_DIR)
        parser.add_argument("--input_dim", default=128, type=int)
        parser.add_argument("--latent_dim", default=64, type=int)
        parser.add_argument("--firstx", default=800, type=int)
        parser.add_argument("--lastx", default=3000, type=int)
        parser.add_argument("--epochs", default=10000, type=int)
        parser.add_argument("--load", action="store_true")
        return parser

if __name__ == "__main__":
    Run(sys.argv[1:])
