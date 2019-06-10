import argparse
import pathlib
import sys
import tensorflow as tf

def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    width = parsed_arguments.width
    height = parsed_arguments.height
    model = build_model(width, height)

    path = parsed_arguments.outfile
    save_model(model, path)

def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('-o', '--outfile', help="Output file")
    parser.add_argument('-w', '--width', help="Width of the map",
                        type=int)
    parser.add_argument('--height', help="Height of the map",
                        type=int)

    args = parser.parse_args(arguments)
    return args

def build_model(width, height):
    model = tf.keras.Sequential()
    model.add(tf.keras.layers.Flatten(input_shape=(width, height)))
    model.add(tf.keras.layers.Dense(100, activation='relu'))
    model.add(tf.keras.layers.Dense(5))
    model.compile(loss='mean_squared_error', optimizer='sgd')
    return model

def save_model(model, path_string):
    frozen_graph = freeze_session(tf.keras.backend.get_session(),
                                  output_names=[out.op.name for out in model.outputs])

    path = pathlib.Path(path_string).absolute()
    tf.io.write_graph(frozen_graph, str(path.parent), path.name, as_text=True)

def freeze_session(session, keep_var_names=None, output_names=None, clear_devices=True):
    graph = session.graph
    with graph.as_default():
        freeze_var_names = list(set(v.op.name for v in tf.global_variables()).difference(keep_var_names or []))
        output_names = output_names or []
        output_names += [v.op.name for v in tf.global_variables()]
        input_graph_def = graph.as_graph_def()
        if clear_devices:
            for node in input_graph_def.node:
                node.device = ""
        frozen_graph = tf.graph_util.convert_variables_to_constants(
            session, input_graph_def, output_names, freeze_var_names)
        return frozen_graph

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))