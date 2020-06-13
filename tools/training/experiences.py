import itertools


class Experience:
    def __init__(self, turn, next_turn, next_next_turn):
        self.turn = turn
        self.next_turn = next_turn
        self.next_next_turn = next_next_turn


def experiences(games):
    def pairwise(iterable):
        a, b, c = itertools.tee(iterable, 3)
        next(b, None)
        next(c, None)
        next(c, None)
        return zip(a, b, c)

    for game in games:
        for turn, next_turn, next_next_turn in pairwise(game.turns()):
            yield Experience(turn, next_turn, next_next_turn)
