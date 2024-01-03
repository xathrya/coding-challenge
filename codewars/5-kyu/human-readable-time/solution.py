# https://www.codewars.com/kata/52685f7382004e774f0001f7

# solution 1: 462ms
def make_readable(seconds):
    hours, seconds = divmod(seconds, 3600)
    minutes, seconds = divmod(seconds, 60)

    return f"{hours:02}:{minutes:02}:{seconds:02}"