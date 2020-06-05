from intro import intro
from strings import strings
from dna_toolkit import gen_random_seq, transcription

if __name__ == "__main__":
    print("Hello, I am Python!")
    # intro()
    # strings()
    dna = gen_random_seq(20)
    print(dna)
    print(transcription(dna))
