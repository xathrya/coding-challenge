# https://www.codewars.com/kata/54dba07f03e88a4cec000caf

# solution 1: 637ms
class Dog:
    def __init__(self, breed):
        self.breed = breed

    def bark(self):
        return "Woof"

snoopy = Dog("Beagle")
scoobydoo = Dog("Great Dane")