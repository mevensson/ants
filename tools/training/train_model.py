#!/usr/bin/env python3
import argparse
import numpy
import os.path
import sys

from experiences import experiences
from game import Game, run_game
from model import Model


def main(arguments):
    parsed_arguments = parse_arguments(arguments)

    model = Model()
    model.create(7, 7)
    model.save(parsed_arguments.model_dir)
    for i in range(parsed_arguments.num_iterations):
        exploration_factor = 0.75 - 0.5 * i / parsed_arguments.num_iterations
        run_game(parsed_arguments.playgame_path,
                 parsed_arguments.map_path,
                 parsed_arguments.log_dir,
                 parsed_arguments.exe_path,
                 parsed_arguments.model_dir,
                 parsed_arguments.turns,
                 parsed_arguments.games,
                 exploration_factor)
        games = load_games(parsed_arguments.log_dir, 1, parsed_arguments.games)
        print(f'Iteration: {i + 1} of {parsed_arguments.num_iterations}')
        print_score(games)
        print(f'Exploration factor: {exploration_factor}')
        for _ in range(3):
            games = load_games(parsed_arguments.log_dir,
                               1, parsed_arguments.games)
            input, output = convert_experiences(experiences(games), model)
            model.train(input, output)
        model.save(parsed_arguments.model_dir)


def parse_arguments(arguments):
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--playgame_path', help="Path to playgame script")
    parser.add_argument('--map_path', help="Path to playgame script")
    parser.add_argument(
        '--log_dir', help="Path to directory where game logs should be saved")
    parser.add_argument('--exe_path', help="Path to bot executable")
    parser.add_argument(
        '--model_dir', help="Path to directory where model should be saved")
    parser.add_argument('--num_iterations',
                        help="Number of iterations",
                        type=int)
    parser.add_argument('--games',
                        help="Number of games each iteration",
                        type=int)
    parser.add_argument('--turns',
                        help="Number of turns each game",
                        type=int)

    args = parser.parse_args(arguments)
    return args


def load_games(directory, bot_num, num_games):
    for game_num in range(num_games):
        yield Game(directory, game_num, bot_num)


def print_score(games):
    sum_scores = 0
    num_scores = 0
    for game in games:
        last_turn = None
        for last_turn in game.turns():
            pass
        if last_turn:
            score = len(last_turn.actions)
            sum_scores += score
            num_scores += 1
    avg_score = sum_scores / num_scores
    print(f'Average score: {avg_score}')


def convert_experiences(experiences, model):
    height = model.height()
    width = model.width()
    input_list = []
    output_list = []
    turn = 1
    for experience in experiences:
        reward = 10 * (len(experience.next_next_turn.actions) -
                       len(experience.next_turn.actions))
        #print(f'Turn: {turn}')
        turn += 1
        for action in experience.turn.actions:
            food_map = create_food_map(
                action['pos'], experience.turn.food, height, width)
            state = numpy.stack((food_map,))
            input_list.append(state)
            next_food_map = create_food_map(
                action['pos'], experience.next_turn.food, height, width)
            next_state = numpy.stack((next_food_map,))
            output = create_output(model, reward, state,
                                   next_state, action['dir'])
            output_list.append(output)
            # print(state)
            # print(output)
    inputs = numpy.stack(input_list)
    outputs = numpy.stack(output_list)
    return (inputs, outputs)


def create_food_map(pos, food_list, height, width):
    result = numpy.zeros((width, height))
    #print(f'Pos: {pos}')
    for food in food_list:
        #print(f'Food: {food}')
        x = food[0] - pos[0] + width // 2
        y = food[1] - pos[1] + height // 2
        if 0 <= x < width and 0 <= y < height:
            result[x, y] = 1
    return result


def create_output(model, reward, state, next_state, dir):
    output = model.predict(numpy.stack((state,)))[0]
    value = reward + 0.9 * \
        numpy.argmax(model.predict(numpy.stack((next_state,))))
    if dir == 'n':
        output[0] = value
    elif dir == 'e':
        output[1] = value
    elif dir == 's':
        output[2] = value
    elif dir == 'w':
        output[3] = value
    else:
        raise SystemExit(f'Error: invalid direction "{dir}"')
    return output


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
