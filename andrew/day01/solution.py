import time
import math


def calculate_max():
    file = open("./input.txt", "r")
    max = 0
    temp_list = []
    for line in file:

        if line == "\n":
            temp_sum = sum(temp_list)
            if temp_sum > max:
                max = temp_sum
            temp_list = []

        else:
            int_line = int(line[:-1])
            temp_list.append(int_line)

    file.close()
    return max


def calculate_max_three():
    file = open("./input.txt", "r")

    max_values = []
    temp_list = []

    for line in file:
        if line == "\n":
            temp_sum = sum(temp_list)
            max_values.append(temp_sum)
            temp_list = []
        else:
            int_line = int(line[:-1])
            temp_list.append(int_line)

    max_values = sorted(max_values, reverse=True)

    file.close()
    return max_values[0] + max_values[1] + max_values[2]


if __name__ == "__main__":
    start_time = time.time()
    print("max_value:", calculate_max())
    print("Function Running Time:", time.time() - start_time, "\n")

    start_time = time.time()

    print("three max values:", calculate_max_three())
    print("Function Running Time:", time.time() - start_time)
