#!/usr/bin/env python3
import argparse
import os.path
import re
import sys


def main(arguments):
    parsed_arguments = parse_arguments(arguments)
    experiences = load_experience(
        parsed_arguments.experience_dir, parsed_arguments.bot, 0)
    print(experiences)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--experience_dir', help="Path to experience dirctory")
    parser.add_argument('--bot', help="Which bot number to train", type=int)

    args = parser.parse_args(arguments)
    return args


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


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
