def strings():
    test_str = "Doom"
    print(test_str)

    test_str += " III"
    print(test_str)

    test_str += '!'
    print(test_str)

    p1 = "Duke"
    p2 = " Nukem"

    p3 = p1 + p2
    print(f"{p1} {p2} {p3}")

    # Print each character:
    for ch in p3:
        print(ch)

    # Print each character and it's index:
    for pos, ch in enumerate(p3):
        print(pos, ch)

    # Print a slice of the string
    print(p3[0:5])
