#!/usr/bin/env python3
import argparse
import pathlib
import sys
import tensorflow as tf


def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    maps = parsed_arguments.maps
    width = parsed_arguments.width
    height = parsed_arguments.height
    output = parsed_arguments.output
    model = build_model(maps, width, height, output)

    path = parsed_arguments.outfile
    save_model(model, path)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('-o', '--outfile', help="Output file")
    parser.add_argument('--maps', help="Number of maps",
                        type=int)
    parser.add_argument('--width', help="Width of the map",
                        type=int)
    parser.add_argument('--height', help="Height of the map",
                        type=int)
    parser.add_argument('--output', help="Size of the output",
                        type=int)

    args = parser.parse_args(arguments)
    return args


def build_model(maps, width, height, output):
    model = tf.keras.Sequential()
    model.add(tf.keras.layers.Input(shape=(maps, width, height)))
    model.add(tf.keras.layers.Flatten())
    model.add(tf.keras.layers.Dense(10, activation='relu'))
    model.add(tf.keras.layers.Dense(output, activation='softmax'))
    model.compile(loss='categorical_crossentropy', optimizer='sgd',
                  metrics=['accuracy'])
    return model


def save_model(model, path_string):
    print(model.summary())
    print("Input name:  " + model.input.op.name)
    print("Output name: " + model.output.op.name)
    tf.compat.v1.keras.utils.plot_model(
        model, to_file='model.png', show_shapes=True)
    tf.saved_model.save(model, path_string)


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
