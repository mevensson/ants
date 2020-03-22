#!/usr/bin/env python3
import argparse
import subprocess
import sys


def main(arguments):
    parsed_arguments = parse_arguments(arguments)
    run_game(parsed_arguments)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--playgame', help="Path to playgame")
    parser.add_argument('--rounds', help="Number of rounds to play")
    parser.add_argument('--log_dir', help="Log firectory")
    parser.add_argument('--map_file', help="Map file to use")
    parser.add_argument('--bot', help="The bot")

    args = parser.parse_args(arguments)
    return args


def run_game(arguments):
    subprocess.run(
        [arguments.playgame,
         "--log_stream",
         "--log_dir", arguments.log_dir,
         "--map_file", arguments.map_file,
         "--turns", "100",
         "--rounds", arguments.rounds,
         arguments.bot,
         arguments.bot]
    )


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
