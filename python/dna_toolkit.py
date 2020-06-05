from random import choice


def gen_random_seq(length):
    nucleuotides = ['A', 'C', 'G', 'T']
    rnd_str = ""

    for _ in range(length):
        rnd_str += choice(nucleuotides)

    return rnd_str


def transcription(dna):
    return dna.replace("T", "U")
