class day_02:
    def readfile(self):
        with open("input.txt", "r") as f:
            lines = f.read().splitlines()
        return lines

    def parse_lines(self, line):
        """Takes a single line and breaks it down by round and colors into a dictionary"""
        game_dict = dict()
        game, colors = line.split(":")
        game = game.split("Game ")
        game_dict["Game"] = int(game[1])
        game_dict["Colors"] = colors
        return game_dict  # gamedict = {"Game":1, "Colors": 1 green, 1 blue, 1 red;"

    def game_possible(self, colors_key, color_max):
        """ "Tests game possibility. Requires all the colors and the allowed color maxes. Returns false if the game is not possible. Returns true if the game is possible."""
        colors = colors_key.split(";")
        for i in colors:
            color_dict = {"red": 0, "green": 0, "blue": 0}
            for j in i.split(","):
                j = j.strip()
                if j:
                    count, color = j.split()
                    color_dict[color] += int(count)
            if (
                color_dict["red"] > color_max["red"]
                or color_dict["green"] > color_max["green"]
                or color_dict["blue"] > color_max["blue"]
            ):
                return False
        return True

    def game_possible_fewest(self, colors_key):
        colors = colors_key.split(";")
        color_min_dict = {"red": 0, "green": 0, "blue": 0}
        for i in colors:
            color_dict = {"red": 0, "green": 0, "blue": 0}
            for j in i.split(","):
                j = j.strip()
                if j:
                    count, color = j.split()
                    color_dict[color] += int(count)
            if color_dict["red"] > color_min_dict["red"]:
                color_min_dict["red"] = color_dict["red"]
            if color_dict["green"] > color_min_dict["green"]:
                color_min_dict["green"] = color_dict["green"]
            if color_dict["blue"] > color_min_dict["blue"]:
                color_min_dict["blue"] = color_dict["blue"]
        power = color_min_dict["red"] * color_min_dict["green"] * color_min_dict["blue"]
        return power

    def part1(self):
        total = 0
        color_max = {"green": 13, "red": 12, "blue": 14}
        lines = self.readfile()
        for l in lines:
            parsed_line = self.parse_lines(l)
            if self.game_possible(parsed_line["Colors"], color_max):
                total += parsed_line["Game"]
        return total

    def part2(self):
        total = 0
        lines = self.readfile()
        for l in lines:
            parsed_line = self.parse_lines(l)
            total += self.game_possible_fewest(parsed_line["Colors"])
        return total


if __name__ == "__main__":
    day_02 = day_02()
    print(f"The answer to part one is: {day_02.part1()}")
    print(f"The answer to part one is: {day_02.part2()}")
