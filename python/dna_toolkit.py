from random import choice


def gen_random_seq(length):
    nucleuotides = ['A', 'C', 'G', 'T']
    rnd_str = ""

    for _ in range(length):
        rnd_str += choice(nucleuotides)

    return rnd_str


def transcription(dna):
    return dna.replace("T", "U")


def reverse_complement(dna):
    """
    Generating a complement string and returning
    reveresed version.
    """
    trans_dict = {'A': 'T', 'T': 'A', 'C': 'G', 'G': 'C'}
    complement_dna = ""

    for nuc in dna:
        complement_dna += trans_dict[nuc]

    return complement_dna[::-1]
