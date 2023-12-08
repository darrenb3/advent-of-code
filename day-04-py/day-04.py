import re


class dayfour:
    def readfile(self, input: str):
        """Accepts a string as the file path and returns parsed lines as a large list."""
        with open(f"{input}", "r") as f:
            lines = f.read().splitlines()
        return lines

    def partone(self):
        """Accepts no input and returns the answer to part 1"""
        lines = self.readfile("input.txt")
        total_points = 0
        for i in lines:
            c = 0
            game, nums = i.split(
                ":"
            )  # game: 'Card 1' nums: ' 41 48 83 86 17 | 83 86  6 31 17  9 48 53'
            nums = nums.strip()
            nums_win, nums_have = nums.split(
                "|"
            )  # splits input into the winning numbers and the numbers to check
            nums_win = nums_win.strip()
            nums_win_list = re.split(
                r"\s+", nums_win
            )  # creates a list of each winning num
            nums_have = nums_have.strip()
            nums_have_list = re.split(
                r"\s+", nums_have
            )  # creates a list of each number to check
            for (
                num
            ) in (
                nums_have_list
            ):  # checks all numbers and if it is a winner increases the counter
                if num in nums_win_list:
                    c += 1
            if (
                c > 0
            ):  # if the number of winning numbers is greater than 0 calculates the total of points from the card and adds it to the total points
                total_points += 2 ** (c - 1)
        return total_points


if __name__ == "__main__":
    dayfour = dayfour()
    print(f"The answer to part one is {dayfour.partone()}")
