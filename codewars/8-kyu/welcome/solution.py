# https://www.codewars.com/kata/577ff15ad648a14b780000e7

# solution 1: 480ms
DATABASE = [ 
    ("english", "Welcome"),
    ("czech", "Vitejte"),
    ("danish", "Velkomst"),
    ("dutch", "Welkom"),
    ("estonian", "Tere tulemast"),
    ("finnish", "Tervetuloa"),
    ("flemish", "Welgekomen"),
    ("french", "Bienvenue"),
    ("german", "Willkommen"),
    ("irish", "Failte"),
    ("italian", "Benvenuto"),
    ("latvian", "Gaidits"),
    ("lithuanian", "Laukiamas"),
    ("polish", "Witamy"),
    ("spanish", "Bienvenido"),
    ("swedish", "Valkommen"),
    ("welsh", "Croeso")
]

def greet(language):
    default_greet = ""
    for lang in DATABASE:
        if lang[0] == language:
            return lang[1]
        if lang[0] == "english":
            default_greet = lang[1]
    
    return default_greet


# solution 2: 493ms
def greet(language):
    return {
        "english": "Welcome",
        "czech": "Vitejte",
        "danish": "Velkomst",
        "dutch": "Welkom",
        "estonian": "Tere tulemast",
        "finnish": "Tervetuloa",
        "flemish": "Welgekomen",
        "french": "Bienvenue",
        "german": "Willkommen",
        "irish": "Failte",
        "italian": "Benvenuto",
        "latvian": "Gaidits",
        "lithuanian": "Laukiamas",
        "polish": "Witamy",
        "spanish": "Bienvenido",
        "swedish": "Valkommen",
        "welsh": "Croeso"
    }.get(language, "Welcome")