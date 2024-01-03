# https://www.codewars.com/kata/5202ef17a402dd033c000009

# solution 1: 469ms
def title_case(title, minor_words=''):
    if title:
        if minor_words is None:
            return title.title()
        my_word = [word.lower() for word in title.split()]
        if my_word[0] in minor_words.lower() or minor_words.lower() in my_word:
            return f'{my_word[0].title()} {" ".join([i.title() if i not in minor_words.lower() else i for i in my_word[1:]])}'.strip()
        return " ".join(my_word).title()
    return ''


# solution 2: 486ms
def title_case(title, minor_words=''):
    title = title.capitalize().split()
    minor_words = minor_words.lower().split()
    return " ".join([word if word in minor_words else word.capitalize() for word in title])