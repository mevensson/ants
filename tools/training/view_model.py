#!/usr/bin/env python3
import argparse
import pathlib
import sys
import tensorflow as tf


def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    input_dir = parsed_arguments.indir
    output_dir = parsed_arguments.outdir
    import_to_tensorboard(input_dir, output_dir)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('-i', '--indir', help="Input directory")
    parser.add_argument('-o', '--outdir', help="Output directory")

    args = parser.parse_args(arguments)
    return args


def import_to_tensorboard(input_dir, output_dir):
    with tf.Session() as sess:
        tf.saved_model.loader.load(sess, ["serve"], input_dir)
        writer = tf.summary.FileWriter(output_dir)
        writer.add_graph(sess.graph)
        print("Model Imported. Visualize by running: "
              "tensorboard --logdir={}".format(output_dir))


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
