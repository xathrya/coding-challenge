# https://www.codewars.com/kata/51f2b4448cadf20ed0000386

# solution 1: 520ms
def remove_url_anchor(url):
    return url.split("#")[0]