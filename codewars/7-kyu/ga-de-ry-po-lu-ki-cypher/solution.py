# https://www.codewars.com/kata/592a6ad46d6c5a62b600003f

# solution 1: 490ms
LUT_ENC = "GADERYPOLUKIgaderypoluki"
LUT_DEC = "AGEDYROPULIKagedyropulik"

def encode(message):
    result = []
    for c in message:
        if c in LUT_ENC:
            result.append(LUT_DEC[LUT_ENC.index(c)])
        else:
            result.append(c)
    return "".join(result)

def decode(message):
    result = []
    for c in message:
        if c in LUT_ENC:
            result.append(LUT_DEC[LUT_ENC.index(c)])
        else:
            result.append(c)
    return "".join(result)