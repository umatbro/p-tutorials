class Rule:
    def __init__(self, rule: str):
        to_stay_alive, resurrect = rule.split('/')
        self.to_keep_alive = set(map(int, to_stay_alive))
        self.to_resurrect = set(map(int, resurrect))

    def __str__(self):
        return '{}/{}'.format(''.join(map(str, sorted(self.to_keep_alive))), ''.join(map(str, sorted(self.to_resurrect))))


RULES = {
    'conway': Rule('23/3'),
    'seeds': Rule('/2'),
    'servets': Rule('/234'),
    'wolfram7e': Rule('012345678/1'),
    'life_with_no_death': Rule('012345678/3'),
    'wolfram': Rule('018/018'),
    'labyrinth': Rule('12345/3'),
    '2x2': Rule('125/36'),
    'amoeba': Rule('1358/357'),
    'diamoeba': Rule('5678/35678'),
    'cities': Rule('2345/45678'),
    'coral': Rule('45678/3'),
}
