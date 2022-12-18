import re
from collections import deque

cache = {}


def dfs(valves, workers, per_min, open_valves, mins_left):
    valves_str = ",".join(sorted(open_valves))

    if mins_left <= 0:
        return 0
    elif ret := cache.get((workers, valves_str, mins_left)):
        return ret

    step = []
    for w in workers:
        next_step1 = (
            [(lead, 0, set()) for lead in valves[w]["leads"]] if w else [(w, 0, set())]
        )
        next_step2 = (
            [(w, valves[w]["rate"], {w})]
            if w and valves[w]["rate"] > 0 and w not in open_valves
            else []
        )
        step.append(next_step1 + next_step2)

    score = 0
    for pos1, per_min1, nvalve1 in step[0]:
        for pos2, per_min2, nvalve2 in step[1]:
            if nvalve1 and nvalve2 and nvalve1 == nvalve2:
                continue

            score = max(
                score,
                dfs(
                    valves,
                    (pos1, pos2),
                    per_min + per_min1 + per_min2,
                    open_valves | nvalve1 | nvalve2,
                    mins_left - 1,
                ),
            )
    score += per_min
    cache[(workers, valves_str, mins_left)] = score
    return score


def part_one():
    with open("input.txt", "r") as f:
        data = f.read()
    valves = {}

    first = None
    for line in data.split("\n"):
        e = (
            re.sub(
                r"Valve (\w+) has flow rate=(\d+); tunnel(s?) lead(s?) to valve(s?) (\w+)",
                r"\1,\2,\6",
                line,
            )
            .replace(" ", "")
            .split(",")
        )
        first = e[0] if not first else first
        valves[e[0]] = {"rate": int(e[1]), "leads": deque(e[2:])}

    print(dfs(valves, ("AA", None), 0, set(), 30))
    print(dfs(valves, ("AA", "AA"), 0, set(), 26))


part_one()
