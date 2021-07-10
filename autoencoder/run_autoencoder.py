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

        print(
            "\nRunning {}, batchsize {}, data_dir {}, input_dim {}, latent_dim {}".format(
                args.model,
                args.batchsize,
                args.data_dir,
                args.input_dim,
                args.latent_dim,
            )
        )
        self.args = args
        self.model = MODELS.get(args.model)(
            args.input_dim, args.latent_dim
        )
        self.__run_encoder()

    def __run_encoder(self):
        data = get_ir_data.get(
            self.args.data_dir,
            self.args.firstx,
            self.args.lastx,
            self.args.input_dim,
        )
        xtrain = data[0]
        print(xtrain)
        self.model.load(xtrain)
        self.model.summary()

    def __get_arg_parser(self):
        parser = argparse.ArgumentParser(description="Run an autoencoder")
        parser.add_argument("--model", "-m", default="simple", choices=MODELS)
        parser.add_argument("--batchsize", "-b", default=64, type=int)
        parser.add_argument("--data_dir", "-d", default=constants.DATA_DIR)
        parser.add_argument("--input_dim", default=128, type=int)
        parser.add_argument("--latent_dim", default=64, type=int)
        parser.add_argument("--firstx", default=800, type=int)
        parser.add_argument("--lastx", default=3000, type=int)
        return parser


Run(sys.argv[1:])
