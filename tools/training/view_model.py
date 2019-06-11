import argparse
import pathlib
import sys
import tensorflow as tf

def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    input_file = parsed_arguments.infile
    output_dir = parsed_arguments.outdir
    import_to_tensorboard(input_file, output_dir)

def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('-i', '--infile', help="Input file")
    parser.add_argument('-o', '--outdir', help="Output directory")

    args = parser.parse_args(arguments)
    return args

def import_to_tensorboard(input_file, output_dir):
    with tf.Session() as sess:
        with tf.gfile.GFile(input_file, 'rb') as f:
            graph_def = tf.GraphDef()
            graph_def.ParseFromString(f.read())
            sess.graph.as_default()
            model = tf.import_graph_def(graph_def)

    writer = tf.summary.FileWriter(output_dir)
    writer.add_graph(sess.graph)
    print("Model Imported. Visualize by running: "
          "tensorboard --logdir={}".format(output_dir))

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
