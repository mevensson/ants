import os.path
import re
import subprocess


class Turn:
    @staticmethod
    def read_turn_0(bot_file, stream_file):
        Turn.__read_bot_turn_0(bot_file)
        Turn.__read_stream_turn_0(stream_file)

    @staticmethod
    def __read_bot_turn_0(file):
        file.readline()

    @staticmethod
    def __read_stream_turn_0(file):
        for line in file:
            m = re.match(r'turn (?P<turn>\d+)', line)
            if m and int(m.group('turn')) == 1:
                return
        raise SystemExit("Error: could not find turn 1")

    @staticmethod
    def read_turn(bot_file, stream_file, bot_num):
        turn = Turn()
        if turn.__read_stream_turn(stream_file, bot_num):
            turn.__read_bot_turn(bot_file)
            return turn

        return None

    def __init__(self):
        self.actions = []
        self.food = []
        self.score = None

    def __read_stream_turn(self, file, bot_num):
        for line in file:
            m = re.match(r'(?P<key>\w+)(?P<args>.*)', line)
            if not m:
                raise SystemExit(
                    f'Error: failed to parse stream line:\n{line}')
            elif m.group('key') == 'end':
                return False
            elif m.group('key') == 'turn':
                return True
            elif m.group('key') == 'score':
                self.score = re.findall(
                    r'\d+', m.group('args'))[bot_num - 1]
            elif m.group('key') == 'f':
                self.food.append(tuple(int(val)
                                       for val in re.findall(r'\d+', m.group('args'))))

    def __read_bot_turn(self, file):
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
                self.actions.append({'pos': pos, 'dir': dir})


class Game:
    def __init__(self, dir, game_num, bot_num):
        self.dir = dir
        self.game_num = game_num
        self.bot_num = bot_num

    def turns(self):
        stream_filename = os.path.join(self.dir, f'{self.game_num}.stream')
        bot_filename = os.path.join(
            self.dir, f'{self.game_num}.bot{self.bot_num}.output')
        with open(stream_filename) as stream_file, open(bot_filename) as bot_file:
            Turn.read_turn_0(bot_file, stream_file)
            while True:
                turn = Turn.read_turn(bot_file, stream_file, self.bot_num)
                if not turn:
                    break
                yield turn


def run_game(playgame_path, map_path, log_dir, exe_path, model_dir, turns, rounds, exploration_factor):
    bot_string = f'{exe_path} -m "{model_dir}" -e {exploration_factor}'
    command_line = [playgame_path,
                    "--log_stream",
                    "--log_output",
                    # "--log_replay",
                    "--log_dir", log_dir,
                    "--map_file", map_path,
                    "--turns", str(turns),
                    "--rounds", str(rounds),
                    bot_string,
                    bot_string]
    subprocess.run(command_line)
