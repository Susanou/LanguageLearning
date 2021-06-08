def convert(number):

    sound = ""

    if int(number) % 3 == 0:
        sound = sound + 'Pling'
    if int(number) % 5 == 0:
        sound = sound + 'Plang'
    if int(number) % 7 == 0:
        sound = sound + 'Plong'
    if int(number) % 3 * int(number) % 5 * int(number) % 7 > 0:
        sound = str(number)

    return sound

    pass
