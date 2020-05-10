#!/usr/bin/env python3
import argparse
import os.path
import numpy
import re
import sys
import tensorflow


def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    experiences = load_experiences(
        parsed_arguments.experience_dir, parsed_arguments.bot, parsed_arguments.num_experiences)
    model = load_model(parsed_arguments.model_dir)
    model = train_model(experiences, model)
    save_model(model, parsed_arguments.model_dir)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--bot', help="Which bot number to train", type=int)
    parser.add_argument('--experience_dir', help="Path to experience dirctory")
    parser.add_argument('--model_dir', help="Path to model dirctory")
    parser.add_argument('--num_experiences',
                        help="Number of experiences to train on", type=int)

    args = parser.parse_args(arguments)
    return args


def load_experiences(directory, bot_num, num_experiences):
    result = []
    for num in range(num_experiences):
        result += load_experience(directory, bot_num, num)
    return result


def load_experience(directory, bot_num, experience_num):
    stream_filename = os.path.join(directory, f'{experience_num}.stream')
    bot_filename = os.path.join(
        directory, f'{experience_num}.bot{bot_num}.output')
    result = []
    with open(stream_filename) as stream_file, open(bot_filename) as bot_file:
        read_stream_turn_0(stream_file)
        read_bot_turn_0(bot_file)
        while True:
            turn = read_stream_turn(stream_file, bot_num)
            if turn:
                turn['actions'] = read_bot_turn(bot_file)
                result.append(turn)
            else:
                break
    return result


def read_stream_turn_0(file):
    for line in file:
        m = re.match(r'turn (?P<turn>\d+)', line)
        if m and int(m.group('turn')) == 1:
            return
    raise SystemExit("Error: could not find turn 1")


def read_bot_turn_0(file):
    file.readline()


def read_stream_turn(file, bot_num):
    result = {'score': None, 'food': []}
    for line in file:
        m = re.match(r'(?P<key>\w+)(?P<args>.*)', line)
        if not m:
            raise SystemExit(f'Error: failed to parse stream line:\n{line}')
        elif m.group('key') == 'end':
            return None
        elif m.group('key') == 'turn':
            break
        elif m.group('key') == 'score':
            result['score'] = re.findall(r'\d+', m.group('args'))[bot_num - 1]
        elif m.group('key') == 'f':
            result['food'].append(tuple(int(val)
                                        for val in re.findall(r'\d+', m.group('args'))))
    return result


def read_bot_turn(file):
    result = []
    for line in file:
        m = re.match(r'(?P<key>\S+)(?P<args>.*)', line)
        if not m:
            raise SystemExit(f'Error: failed to parse bot line:\n{line}')
        elif m.group('key') == '#':
            break
        elif m.group('key') == 'o':
            pos = tuple(int(val)
                        for val in re.findall(r'\d+', m.group('args')))
            dir = re.findall(r'[ensw]', m.group('args'))[0]
            result.append({'pos': pos, 'dir': dir})
    return result


def load_model(dir):
    model = tensorflow.keras.models.load_model(dir)
    return model


def train_model(experiences, model):
    height = model.inputs[0].shape[3]
    width = model.inputs[0].shape[2]
    input, output = convert_experiences(experiences, height, width)
    es = tensorflow.keras.callbacks.EarlyStopping(
        monitor='val_acc', mode='auto', verbose=1)
    model.fit(input, output,
              epochs=100,
              validation_split=0.3,
              verbose=2,
              callbacks=[es])
    return model


def convert_experiences(experiences, height, width):
    input_list = []
    output_list = []
    for experience in experiences:
        food_map = create_food_map(experience['food'], height, width)
        for action in experience['actions']:
            ant_map = create_ant_map(action['pos'], height, width)
            input_list.append(numpy.stack((ant_map, food_map)))
            output_list.append(create_action(action['dir']))
    inputs = numpy.stack(input_list)
    outputs = numpy.stack(output_list)
    return (inputs, outputs)


def create_food_map(food_list, height, width):
    result = numpy.zeros((width, height))
    for food in food_list:
        result[food[0], food[1]] = 1
    return result


def create_ant_map(pos, height, width):
    result = numpy.zeros((width, height))
    result[pos[0], pos[1]] = 1
    return result


def create_action(dir):
    result = numpy.zeros(4)
    if dir == 'n':
        result[0] = 1
    elif dir == 'e':
        result[1] = 1
    elif dir == 's':
        result[2] = 1
    elif dir == 'w':
        result[3] = 1
    return result


def save_model(model, dir):
    tensorflow.saved_model.save(model, dir)


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
