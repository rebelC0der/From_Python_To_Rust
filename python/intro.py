def intro():
    x = 100
    y = 1.0

    print(f"x: {x}, and y: {y}")
    y = y * -0.3145
    print(f"x: {x}, and y: {y}")

    if y < x:
        print(f"The difference is: {x-y}")
    elif y == x:
        print(f"The difference is: {x-y}")
    else:
        print(f"The difference is: {y-x}")

    # x = 5
    # while x > 0:
    #     print(f"x is {x}")
    #     x -= 1

    for i in range(3, 7):
        print(f"i is {i}")

    tmp_lst = ['DNA', 'RNA', 'mRNA']
    for pos, value in enumerate(tmp_lst):
        print(f"value at pos {pos} is {value}")

    print(tmp_lst[0])
