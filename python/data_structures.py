def dicts():
    test_dict = {}
    test_dict['Key_1'] = 'Value_1'
    test_dict['Key_2'] = 'Value_2'
    test_dict['Key_3'] = ['Value_1', 'Value_2']

    print(test_dict['Key_1'])

    if test_dict['Key_3']:
        print(test_dict['Key_3'])
        print(test_dict['Key_3'][1])

    for key, value in test_dict.items():
        print(key, value)
