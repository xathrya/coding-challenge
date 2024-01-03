# https://www.codewars.com/kata/5863f97fb3a675d9a700003f

# solution 1: 521ms
def sum_ppg(player_one, player_two):
    return player_one.get("ppg", 0) + player_two.get("ppg", 0)