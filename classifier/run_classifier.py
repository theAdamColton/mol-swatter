import sys
import argparse
import constants
sys.paths.append(constants.BASE_DIR + "/models/")
import models


MODELS = {
        "simple": models.supervised_contrastive.Classifier
        }

def run():
    """Test or train the classifier"""
    pass


def __get_arg_parser():
    parser = argparse.ArgumentParser(description="Test or train this classifier")
    parser.add_argument("--model", "-m", default="simple", choices=MODELS)
    parser.add_argument("--model", "-m", default="simple", choices=MODELS)
    parser.add_argument("--batchsize", "-b", default=512, type=int)
    parser.add_argument("--data_dir", "-d", default=DATA_DIR)
    parser.add_argument("--autoencoder_dir", default=constants.AUTOENCODER_DIR)

