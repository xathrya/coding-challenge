# https://www.codewars.com/kata/56bcaedfcf6b7f2125001118

# solution 1: 525ms
def html_special_chars(data): 
    return data.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")