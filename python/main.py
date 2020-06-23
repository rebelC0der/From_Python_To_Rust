from intro import intro
from strings import strings
from dna_toolkit import gen_random_seq, transcription, reverse_complement
from data_structures import dicts
from dna_toolkit import reverse_complement


if __name__ == "__main__":
    print("Hello, I am Python!")
    # intro()
    # strings()
    dna = gen_random_seq(20)
    print(dna)
    # print(transcription(dna))
    # dicts()
    print(reverse_complement(dna))
